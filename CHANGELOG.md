# Changelog

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
