# Changelog

## 1.0.0

Breaking, and the surface is settled enough to say so.

### Changed

- **Renamed from `tfl-cli` to `tfl-mcp`**, including the published image, which
  moves to `ghcr.io/radiosilence/tfl-mcp`. Its siblings earn the `-cli` suffix
  — you do want to send mail from a terminal — but nobody checks the tube from
  a shell when they could ask an assistant. The binary is still `tfl`.

- **Serving is the default.** `tfl --http 0.0.0.0:8080` replaces `tfl mcp
  --http …`, and the `mcp` subcommand is gone rather than deprecated. Anything
  invoking it must adapt; keeping a compatibility alias alive from the first
  day of a 1.0 sets the wrong precedent. Deployments must therefore move their
  image tag and their arguments together.
- **Removed the `arrivals`, `status` and `search` subcommands.** They were
  GraphQL queries assembled by pasting strings together: a second
  implementation of what `tfl query` already does, carrying its own escaping so
  that a station called `King's Cross` could not end the literal early. Nobody
  would type them when they could ask an assistant.
- **Every read now takes the same path** — resolver, loader, request. The
  argument-less feeds (the `Meta` vocabularies, roads, air quality, charge
  connectors, car parks, bike points) went straight to the client, which meant
  two branches of one query each fetched them. They share a loader now, so
  `{ a: modes { name } b: modes { name } }` costs one request rather than two.
  Closes the last case where identical reads in a single query did not
  deduplicate.

## 0.5.1

### Fixed

- `StopDisruption.isBlocked` could only ever return `false` — both branches of
  its expression produced the same value. "Is this station closed" answered
  confidently and wrongly, with no error. Replaced by `isClosed` and
  `closureText`, derived from the fields that actually carry closure state,
  and documented so a partial closure is not read as a shut station.
- The MCP tool description and instructions still listed only the domains that
  existed two releases ago, so nothing signalled that roads, air quality,
  taxis, charge points, car parks or collision history were reachable at all.
- `vehicleArrivals` claimed to batch at 20 and did not chunk. Now sent 25 per
  request, TfL's documented maximum.
- `accidents` claimed "nearest first" but ranked every record equally when no
  coordinate was given, making `first` an arbitrary slice. It now orders by
  date in that case and says so.
- `journey`'s `accessibility` argument listed three of TfL's six accepted
  values, so "avoid escalators" had no visible way to be asked for.
- `severities` did not say it is the vocabulary for *lines* only; road
  disruptions grade in words. Added `roadSeverities`.
- `carParks` now says TfL carries no coordinates on that feed, so "the nearest
  car park" is knowingly unanswerable rather than quietly wrong.
- Complexity multipliers added to `chargeConnectors` and `carParks`, the two
  largest unfiltered lists, which carried none.
- A failed fetch no longer reads as a confident empty answer. `load_batch`'s
  retry path discarded every error and returned `Ok` regardless, so a revoked
  key produced twenty nulls rather than a failure; and a per-key failure was
  dropped from the loader's map, which every resolver turned into an empty
  list — so one stop's arrivals timing out beside working siblings reported
  "no trains due", and a failed disruption fetch reported good service.
- The journey rescue search never ran when TfL returned no candidates, which
  is the case it exists for.

## 0.5.0

### Added

- `StopPoint.directionTo` — inbound or outbound to reach another stop. Every
  field taking a `direction` argument wants exactly this, and a model was
  previously guessing.
- `StopPoint.canReachOnLine` — where you can get to without changing.
- `now` — the current time in London, with offset and whether the tube is
  likely running. TfL's timestamps are London-local with no zone marker, so
  "is this departure soon" was unanswerable without knowing what time it is
  there. Costs no request.

### Fixed

- TfL declares booleans it then sends as strings — `StopPoint.status` comes
  back as `"Unknown"` from `/CanReachOnLine`. serde fails a whole response on
  one bad field, so a single stop with an opinion about its status lost the
  other fourteen. Generated booleans now accept either, and an unrecognised
  word decodes as null rather than guessing `false`.
- List decoding no longer hides the real error. `#[serde(untagged)]` reported
  only "data did not match any variant", throwing away which field and which
  line — every decode failure in the client had become undiagnosable.

## 0.4.0

### Added

- `Line.route(direction:)` — the stops on a line **in travel order**, with
  branches, stop counts and per-station zones. This is what answers "how many
  stops to Oxford Circus" and "am I going the right way"; `stopPoints` returns
  the same stations unordered and cannot answer either.
- `StopPoint.disruptions` — a closed entrance or a broken lift, as distinct
  from the disruptions affecting the lines that call there.

### Changed

- Reference data is now cached whatever the configuration says, and live data
  still only when asked for. Deciding by what an endpoint *is* rather than by
  its TTL removes the footgun entirely: no setting can make an arrival stale,
  and the vocabulary a model reads before writing its first query stops costing
  a request every time.
- README no longer implies endpoint-level coverage. 84 endpoints exist, many
  are variants of one another, and the graph reaches what answers questions
  rather than one field per endpoint.

## 0.3.1

### Fixed

- **A query could fan out to hundreds of concurrent requests.** Depth was
  capped but width was not, so `linesByMode(modes: ["bus"]) { stopPoints }` —
  two levels deep, 676 lines wide — would have fired 676 simultaneous requests
  and burned a minute of TfL's rate limit in one call. Fan-out fields now
  declare their width and the schema refuses the product; the client also caps
  requests in flight, which guards paths nobody anticipated.
- Complexity costs overflowed a few levels of nesting in. Debug panicked;
  release wrapped to a small number, so the limit silently stopped limiting.
  Each field's cost is now clamped.
- `hasBikes`/`hasDocks` ignored whether a station was locked or uninstalled.
  TfL leaves stale counts on stations it has pulled, so `bikePointsNear` could
  send someone to a locked dock reporting four bikes.
- An ambiguous journey with no usable candidates read as a confident "no route
  exists" rather than as ambiguous. Ambiguity is now recorded when TfL says so
  rather than inferred from the candidate list being non-empty.
- The response cache and the startup credential check were built and never
  wired to anything. `TFL_CACHE=1` enables the cache; a bad app key now fails
  when the server starts rather than as unexplained 429s on the first query.
- Poisoned-mutex tolerance: one panicking request no longer bricks every
  subsequent one.

## 0.3.0

### Added

- Roads: corridors and disruptions, worst-first, with `corridorIds` resolved
  into the roads a disruption blocks.
- Air quality: forecast bands per pollutant, with TfL's escaped HTML decoded.
- Cabwise: licensed taxi and minicab operators near a point.
- Occupancy: EV charge connectors (batched) and car parks.
- AccidentStats: 2019 casualty records, filtered locally by radius, severity
  and borough because TfL offers no filter of its own.

### Fixed

- The generator emitted an empty struct for `System.Object`, which parsed
  successfully and discarded the entire payload. Definitions with no properties
  now decode as raw JSON, which is what made air quality and Cabwise reachable
  at all.

## 0.2.1

### Fixed

- Tool descriptions and server instructions never mentioned journey planning or
  Santander Cycles, which landed after they were written — so a model had no way
  to know either existed. The tool description matters most: it is always
  loaded, and is what decides whether the server gets reached for at all.

## 0.2.0

### Added

- GraphQL schema over the TfL Unified API, joining TfL's foreign keys into a
  graph: `Prediction.line`/`.destination`/`.stopPoint`, `StopPoint.lines`/
  `.arrivals`, `Line.stopPoints`/`.disruptions`. Every edge resolves through a
  DataLoader, so following one across a list is a single batched request.
- MCP server over stdio and streamable-HTTP, exposing `tfl_schema` and `tfl`.
  GraphQL and GraphiQL are available on the same listener.
- `tfl-api-client`, generated from TfL's Swagger document by `cargo xtask regen`.
- CLI: `arrivals`, `status`, `search`, `query`, `schema`, `mcp`, `completions`.
- Journey planning: `journey(from:, to:)` with legs, changes, fares, obstacles
  and accessibility preferences. Handles TfL's `300 Multiple Choices` by
  returning candidate locations rather than failing.
- Santander Cycles: `bikePoint`, `bikePointsNear`, `searchBikePoints`, with
  TfL's property bag parsed into typed counts and a batched occupancy edge.

### Notes

- Loaders cache within a request as well as batching. Batching alone only
  collapses keys that arrive in the same window, so two branches of one query
  asking for the same stop would each fetch it.

- Caching is off by default; transit data goes stale within seconds. When
  enabled it honours only TfL's own `Cache-Control`.
- A blank `app_key` is never sent: TfL answers an invalid key with 429 where an
  anonymous caller gets 200, so sending one is strictly worse than sending none.
