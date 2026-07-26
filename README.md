# tfl-mcp

London transport as a GraphQL API, and an MCP server over it. Live arrivals,
line status, disruptions, journeys, roads, bikes and air quality.

It is a server, not a CLI. The binary serves; the two subcommands exist to
inspect what it serves:

```console
$ tfl                                        # MCP over stdio
$ tfl --http 0.0.0.0:8080 --graphql          # MCP + GraphQL over HTTP
$ tfl --http 127.0.0.1:8080 --graphiql       # + GraphiQL, opens a browser

$ tfl schema                                 # the SDL a model reads
$ tfl query '{ disruptedLines { name statuses { description } } }'
```

There were `arrivals`, `status` and `search` subcommands. They were GraphQL
queries built by pasting strings together, which nobody would type when they
could ask an assistant, and which needed their own escaping to keep a station
called `King's Cross` from ending the literal early. `tfl query` does the same
job without a second implementation to keep honest.

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
carry their own disruptions.

A place name usually needs resolving first. TfL treats "Kings Cross" as
ambiguous and answers with candidates instead of routes, so ask for both and
you learn which it was in one round trip:

```graphql
{ journey(from: "Kings Cross", to: "Brixton", preference: "LeastWalking") {
    isAmbiguous
    fromOptions { name value isStation }
    journeys { duration changes fare { totalCost }
      legs { mode duration summary isDisrupted
             disruptions { description } } } } }
```

`isAmbiguous: true` means `journeys` is empty and the options are the answer —
stations first, so `fromOptions[0].value` is usually the one meant. Pass it back
as `from`, and the second query returns routes:

```graphql
{ journey(from: "940GZZLUKSX", to: "940GZZLUBXN") {
    journeys { duration changes fare { totalCost }
      legs { mode duration summary } } } }
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
{ roadDisruptions(first: 5) {
    severity category location currentUpdate hasClosures
    roads { displayName status } } }
```

Worst first, so the top of the list is what matters without naming a severity.
There is usually nothing `Serious` on London's roads — a typical afternoon is
around ninety `Minimal` and a dozen `Moderate` — so filtering to the dramatic
ones mostly returns an empty list.

## One question, one query

The compound examples above are the everyday case. This is what the graph is
actually capable of — a whole picture of getting somewhere, in a single
request:

```graphql
{
  now { local weekday tubeLikelyRunning }

  station: stopPoint(id: "940GZZLUKSX") {
    commonName
    crowding { relativeToNormal description }
    disruptions { isClosed description }
    arrivals(first: 3) {
      timeToStation towards platformName
      line { name isGoodService statuses { description reason } }
      destination { commonName lat lon }
    }
    directionTo(toStopPointId: "940GZZLUBXN", lineId: "victoria")
  }

  victoria: line(id: "victoria") {
    timetable(from: "940GZZLUVIC", direction: "inbound") {
      schedules { name last { time isNextDay } }
    }
  }

  bikes: bikePointsNear(lat: 51.5308, lon: -0.1238, radius: 500, first: 2) {
    commonName distance eBikes emptyDocks
  }

  air: airQuality { current { band nitrogenDioxide } }

  roads: roadDisruptions(first: 2) {
    severity location currentUpdate
    roads { displayName status }
  }
}
```

Seven domains — live arrivals, crowding, station closures, line status,
scheduled departures, cycle hire, air quality and road traffic — resolved
together. Real output, abridged:

```json
{"now": {"weekday": "Sunday", "tubeLikelyRunning": true},
 "station": {"commonName": "King's Cross & St Pancras International",
   "crowding": {"description": "much quieter than usual"},
   "directionTo": "inbound",
   "arrivals": [{"timeToStation": 10, "towards": "Walthamstow Central",
     "platformName": "Northbound - Platform 3",
     "line": {"name": "Victoria", "isGoodService": false,
       "statuses": [{"description": "Part Suspended",
         "reason": "No service between Victoria and Brixton while we fix a points failure…"}]},
     "destination": {"commonName": "Walthamstow Central"}}]},
 "victoria": {"timetable": {"schedules": [
   {"name": "Saturday (also Good Friday)", "last": {"time": "03:13", "isNextDay": true}}]}},
 "bikes": [{"commonName": "Birkenhead Street, King's Cross", "distance": 123.5, "emptyDocks": 4}],
 "air": {"current": {"band": "Low"}}}
```

**Twelve upstream requests.** The number matters less than the shape: these are
mostly distinct endpoints, so there is not much to collapse — what the graph
buys here is that a model asks *once* and reasons on a complete answer, rather
than making a dozen sequential tool calls and holding the state between them.
Where there *is* fan-out, it collapses: those three arrivals resolve their lines
and destinations in one batched request each, and would still be one apiece at
forty arrivals.

Note the schema refuses to run this if you make it genuinely pathological —
`linesByMode(modes: ["bus"]) { stopPoints }` is only two levels deep but 676
lines wide, and is rejected before a single request goes out.

## Coverage

Every domain in TfL's spec, bar one — though "domain" is doing real work in
that sentence. TfL publishes 84 endpoints and many are variants of each other
(`/Line/{ids}` and `/Line/{ids}/Status` return the same lines; the second also
carries status, so only it is used). The graph reaches what it needs to answer
questions, not one field per endpoint.

| Domain | What you can ask | Notable |
|---|---|---|
| **StopPoint** | arrivals, disruptions, crowding, facilities, search, geo | live crowding is undocumented by TfL |
| **Line** | status, disruptions, stops **in travel order**, timetables | `route` orders them; `timetable` times them |
| **Journey** | planning, legs, fares, obstacles, step-free routing | takes place names, not just ids |
| **BikePoint** | Santander Cycles: bikes, e-bikes, free docks, nearest | counts parsed out of a stringly-typed bag |
| **Occupancy** | EV charge connectors, car parks | car parks 500 from TfL more often than not |
| **Road** | corridors, disruptions, closures, severity | free text a person wrote, not a code |
| **Place** | car parks, taxi ranks, cycle parks, coach bays, charge stations | everything TfL maps that is not a stop |
| **AirQuality** | pollution forecast by pollutant | HTML decoded out of it |
| **Cabwise** | licensed minicab and taxi operators | for when the tube has stopped |
| **AccidentStats** | 2019 casualty records, by area or radius | 37MB, one year only, filtered locally |
| **Mode**, **Vehicle**, **Search** | vocabulary, vehicle arrivals, stop search | |
| ~~TravelTime~~ | — | deliberately absent: map tile images |

A test derives that list from the committed spec, so a domain cannot go missing
unnoticed and TfL adding one fails the build rather than passing silently.

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
| `crates/tfl-mcp/src/mcp/graphql` | Hand-written. The graph, and the loaders behind it. |
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
