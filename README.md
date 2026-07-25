# tfl-cli

London transport as a GraphQL API, and an MCP server over it. Live arrivals,
line status, disruptions, stations and routes.

```console
$ tfl status --disrupted
$ tfl arrivals "Kings Cross" --limit 5
$ tfl query '{ stopPoint(id: "940GZZLUKSX") { arrivals(first: 3) { timeToStation line { name } } } }'
```

## Why a graph

TfL's Unified API is 84 REST endpoints returning flat records full of foreign
keys. A `Prediction` knows it belongs to `lineId: "victoria"` and terminates at
`destinationNaptanId: "940GZZLUBXN"` — but following either is another request
you have to know how to make, and there are around sixty predictions at a busy
interchange.

Those keys are the graph. Turning them into edges means one query answers what
would otherwise be a fetch-and-loop, and because each edge resolves through a
DataLoader, following the same edge across a list costs one request rather than
one per item:

```graphql
{ stopPoint(id: "940GZZLUKSX") {
    commonName
    arrivals(first: 40) {
      timeToStation
      line { name statuses { description } }
      destination { commonName }
    } } }
```

Forty arrivals across six lines and fifteen destinations: **4 upstream
requests**, where the naive walk is 81. That matters more than it sounds —
anonymous callers get 50 requests/minute, so the naive version exhausts the
budget inside a single question.

## Journeys and bikes

`journey(from: "Kings Cross", to: "Brixton")` takes whatever you have — a name,
a NaPTAN id, a postcode, a `lat,lon` pair — and returns routes with legs, times,
changes and fares.

Names are the normal case and TfL handles them badly. Anything ambiguous comes
back as `300 Multiple Choices`, and for "kings cross" its candidates are eleven
points of interest and four bus stops — Kings Cross Tandoori, the Comfort Inn —
with no King's Cross station among them. So the ambiguous case also searches
stop points for the same term and ranks stations first, then swaps interchange
hubs for a child the planner will actually accept (`HUBKGX` is rejected;
`940GZZLUKSX` is not). Without that, the obvious request is unanswerable from
the options given.

Bike points arrive as a generic place with the interesting part stringly-typed
in a property bag — `NbBikes=4`, `NbEBikes=1`. Those become real fields, so
"is there an e-bike near me" is a query rather than a parsing exercise:

```graphql
{ bikePointsNear(lat: 51.5292, lon: -0.1099, radius: 400) {
    commonName distance bikes eBikes emptyDocks } }
```

TfL offers no geographic filter for bike points, so the ~800 stations are
fetched once per query and measured locally. `bikePointsNear` defaults to
stations that actually have a bike; pass `withDocks: true` when you are
returning one instead.

## What it can answer

The point of joining everything into one graph is compound questions — the ones
that would otherwise be four API calls and a spreadsheet.

**"Should I cycle or take the tube?"** — line status, air quality and bike
availability in one query:

```graphql
{ line(id: "victoria") { isGoodService statuses { description reason } }
  airQuality { current { band nitrogenDioxide } }
  bikePointsNear(lat: 51.5292, lon: -0.1099, radius: 400) {
    commonName distance eBikes emptyDocks } }
```

**"How do I get there, and is anything in the way?"** — a journey whose legs
carry their own disruptions:

```graphql
{ journey(from: "Kings Cross", to: "Brixton", preference: "LeastWalking") {
    journeys { duration changes fare { totalCost }
      legs { mode duration summary isDisrupted
             disruptions { description } } } } }
```

**"Step-free, please."** — accessibility is a first-class argument, and legs
report the stairs they know about:

```graphql
{ journey(from: "940GZZLUKSX", to: "940GZZLUBXN",
          accessibility: ["stepFreeToPlatform", "noSolidStairs"]) {
    journeys { duration legs { mode summary
      obstacles { kind incline position } } } } }
```

**"Is this junction dangerous?"** — 2019 casualty records, filtered to a radius:

```graphql
{ accidents(lat: 51.5152, lon: -0.1418, radius: 200,
            severities: ["Fatal", "Serious"]) {
    date location severity distance
    casualties { class mode severity } vehicles } }
```

**"The tube's stopped, get me home."** — disrupted lines, then a fallback:

```graphql
{ disruptedLines { name statuses { description reason } }
  taxiOperators(lat: 51.5033, lon: -0.1145, openTwentyFourHours: true) {
    name phone } }
```

**"Why is the traffic like this?"** — road disruptions, worst first, each
resolving the corridors it blocks:

```graphql
{ roadDisruptions(severities: ["Serious", "Severe"], first: 5) {
    severity category location currentUpdate hasClosures
    roads { displayName status } } }
```

## Coverage

Every domain in TfL's spec, bar one — though "domain" is doing real work in
that sentence. TfL publishes 84 endpoints and many are variants of each other
(`/Line/{ids}` and `/Line/{ids}/Status` return the same lines; the second also
carries status, so only it is used). The graph reaches what it needs to answer
questions, not one field per endpoint.

| | |
|---|---|
| StopPoint, Line, Prediction | arrivals, status, disruptions, routes, search, geo |
| Journey | planning, legs, fares, obstacles, accessibility, disambiguation |
| BikePoint, Occupancy | Santander Cycles, EV charge connectors, car parks |
| Road | corridors, disruptions, closures |
| AirQuality, Cabwise, AccidentStats | forecasts, licensed operators, casualty history |

**TravelTime is deliberately absent.** It returns map tile images, which mean
nothing over a text protocol.

Known gaps, tracked as issues: timetables (scheduled departures — "when is the
last train"), station crowding by time of day, and reachability between two
stops on a line.

Two things TfL does badly that are worth knowing: `/Occupancy/CarPark` returns
a 500 more often than not — an error there is theirs — and `AccidentStats` only
has 2019, downloads thirty-seven megabytes with no server-side filter, and so is
filtered here after the fact. That field says as much in its own description.

## Layout

| | |
|---|---|
| `crates/tfl-api-client/src/generated` | Generated from TfL's Swagger document. Never hand-edited. |
| `crates/tfl-api-client` | Hand-written transport: auth, retries, errors. |
| `crates/tfl-cli/src/mcp/graphql` | Hand-written. The graph, and the loaders behind it. |
| `xtask` | `cargo xtask regen`. |

The split is the point: the Swagger document is the source of truth for the REST
client and nothing else. How the records join up is not in the spec and cannot
be derived from it, so the schema is ours.

## Codegen

`cargo xtask regen` reads TfL's spec and writes the client. It is pure Rust with
no external tooling — no JVM, no Node — and commits both the spec and the output,
so a TfL change lands as a reviewable diff rather than a surprise.

The generator is bespoke, which is unusual enough to justify.
`openapi-generator` needs a JVM, fails TfL's own spec on validation, renders
`Tfl.Api.Presentation.Entities.StopPoint` as
`TflPeriodApiPeriodPresentationPeriodEntitiesPeriodStopPoint`, and emits
`.join(",").as_ref()` for comma-joined path arrays — which does not compile, on
exactly the batch endpoints that matter most. `progenitor` is pure Rust but
speaks only OpenAPI 3.0, so it would trade the JVM for Node.

Writing ~400 lines instead is only reasonable because TfL uses a tiny corner of
Swagger 2.0 — no `allOf`/`oneOf`/`discriminator`, no inline schemas, path and
query parameters only — and the document has not meaningfully changed in years.
Anything unhandled makes the generator bail rather than emit broken code.

## Running it

```console
$ tfl mcp                                    # stdio, for a local MCP client
$ tfl mcp --http 0.0.0.0:8080                # streamable-HTTP
$ tfl mcp --http 127.0.0.1:8080 --graphiql   # + GraphiQL at /
```

Two tools: `tfl_schema` returns the SDL, `tfl` runs a query. The schema stays
behind a tool call rather than riding in the always-loaded descriptions, so a
session that never mentions a train pays almost nothing for having this
connected.

## Credentials

None required. TfL allows anonymous callers 50 requests/minute, which is enough
to develop against all evening. A key raises it to 500:

1. Register at <https://api-portal.tfl.gov.uk> and activate via email.
2. **Products** → subscribe to the free 500/minute plan.
3. **Profile** → copy the Primary key.

Then `TFL_APP_KEY=...`, or send `X-Tfl-App-Key` per request over HTTP.

Two things worth knowing, neither of which is documented:

- **TfL accepts `app_key` as a header**, not only as a query parameter, so the
  secret stays out of URLs and logs.
- **An invalid key is worse than no key.** TfL answers an unrecognised key with
  `429 Invalid app_key is provided.` where an anonymous caller would have got a
  `200`. A blank key — an unset environment variable — is therefore dropped
  rather than sent, and a rejected key is reported as itself rather than
  retried as throttling.

## Caching

Off by default. Transit data goes stale in seconds and a cached "3 minutes away"
is worse than no answer; within a request the DataLoaders already collapse
duplicate reads, which is where nearly all of the win is.

When enabled it honours only TfL's own `Cache-Control`, never a lifetime we
invented — twelve hours for `/Line/Meta/Modes`, thirty seconds for line status.

## Tests

```console
$ cargo test --workspace              # offline
$ cargo test -- --ignored             # hits the live API, anonymously
```

The live tests cover what a unit test cannot: that batching really does return
every id from one request, that arrivals still carry the foreign keys the graph
is built on, and that an unknown id fails loudly rather than decoding into
garbage.

## Related

Sibling projects sharing this shape: [fastmail-cli], [caldav-cli],
[mainlynorfolk-cli]. All are fronted by [jaritanet-mcp-gateway], which
authenticates the user and injects credentials as per-request headers.

[fastmail-cli]: https://github.com/radiosilence/fastmail-cli
[caldav-cli]: https://github.com/radiosilence/caldav-cli
[mainlynorfolk-cli]: https://github.com/radiosilence/mainlynorfolk-cli
[jaritanet-mcp-gateway]: https://github.com/radiosilence/jaritanet-mcp-gateway
