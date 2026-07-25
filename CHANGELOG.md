# Changelog

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
