# Changelog

## [0.2.5] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.2.4] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.2.3] - 2026-05-17

- Added SAFETY comments to all unsafe FFI calls in `DataFrame::shape()`, `DataFrame::replace_with()`, and `Drop` impl for correctness audit.
- Documented unsafe impl `Send` and `Sync` for `DataFrame` with SAFETY comments explaining the FFI contract guarantees thread-safety.
- Added SAFETY comments to `take_owned_c_string()` error handling in `error.rs` explaining pointer ownership and lifetime.

## [0.2.2] - 2026-05-17

- Added wholesale typed-column protocol coverage with `ColumnId`, `ColumnPrototype`, `ColumnProtocol`, `OptionalColumnProtocol`, `AnyColumnProtocol`, `AnyColumnSlice`, and `DiscontiguousColumnSlice`, plus `Date`/`Data` column constructors and statistics helpers.
- Added broad `DataFrame` mutation helpers: column insert/replace/remove, row-mask filtering, value appends, frame appends, typed transforms/combine helpers, column-id lookups, and array-column explode support.
- Added group-summary expansion (`GroupSummaries`, group filtering/mapping/random split, group counts/ungrouped materialization), CSV-in-memory reading, JSON string helpers, `ShapedData<T>`, and SFrame directory import requests/errors.
- Expanded examples/tests to 18 each and refreshed the coverage docs/audit for the full v0.2.2 surface.

## [0.2.1] - 2026-05-16

- Added JSON read/write support with typed JSON hints, configurable date parsing/formatting, in-memory JSON bytes/strings, and a dedicated JSON example/test pair.
- Added `DataFrame` metadata and reshape helpers for column lookup, aliases, empty-row append/remove, deterministic random splits, and string-formatted descriptions.
- Updated the coverage audit to verify the new JSON, split, formatting, and already-existing sort/row/column helpers while leaving larger typed-protocol/ShapedData gaps explicitly documented.

## [0.2.0] - 2026-05-16

- Expanded the bridge from the original v0.1.0 surface to 13 logical areas: `DataFrame`, `ColumnSlice`, `ColumnEncoder`, `Filter`, `GroupBy`, `Join`, `AnyColumn`, `Sort`, `CSVReader`, `CSVWriter`, `AnyRow`, `Summary`, and `Slicing`.
- Added per-area Swift bridge files and Rust modules using the retained-box `@_cdecl` bridge pattern used elsewhere in the ecosystem.
- Added heterogeneous row/column snapshots (`AnyRow`, `AnyColumn`, `AnyValue`) plus row mutation helpers and `DataFrame::from_rows`.
- Added filter, group-by, join, sort, slicing, column encoding, summary, and CSV projection APIs.
- Added one example and one integration test for each logical area, plus `COVERAGE.md` documenting the implemented and intentionally skipped TabularData surface.

## [0.1.0] - 2026-05-16

- Initial release of `tabulardata-rs`.
- Safe Rust wrappers for `DataFrame`, column construction, CSV reading/writing options, joins, and summary generation.
- SwiftPM bridge for the pure-Swift `TabularData.framework` surface.
- Smoke example that writes `target/tabular.csv`, reloads it, and joins two frames without using `/tmp`.
