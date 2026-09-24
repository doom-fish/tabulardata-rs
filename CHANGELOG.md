# Changelog

All notable changes to `tabulardata-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - Unreleased

### Security

- Unknown column names no longer reach `TabularData`'s `fatalError`, which aborted the process, in filters, sorts, selections, summaries, joins, groupings, group lookups, stratified splits and column encoding; they return `InvalidArgument`.
- Appended, inserted and replaced rows, `from_rows` and `append_rows_of` no longer hit `TabularData`'s element-type traps: every value is converted to its column's element type or rejected, and every column is filled.
- Group aggregates on the wrong element type, time grouping on a non-Date column, joins with mismatched key types, renames onto an existing name, encoding or decoding a column of the wrong type, aliasing an alias, split proportions of exactly 0 or 1, non-ASCII CSV delimiters or escape characters, and overflowing Int group sums no longer abort the process.
- `json_bytes`, `json_string` and `write_json` no longer abort the process with an uncaught Objective-C exception on NaN or infinite Double and Float values, Data values, or dates, data and NaN inside array and object columns; they return `InvalidArgument`. A `summary()` of a one-row column or a quantile of an all-nil group produced such frames.
- Ordered group aggregates no longer abort (or sort by the wrong column) when a grouping column already has the name of the result column (`count`, `sum(x)`, `mean(x)`, `quantile(x)`, `min(x)`, `max(x)`); they return `InvalidArgument`. Unordered aggregates still work and name the result `<name>.1`.

### Fixed

- Group means and quantiles work on Int columns (converted to Double); sums, minimums and maximums follow the column's element type; group lookup keys are converted to the grouping column types; quantiles outside 0...1 are rejected.
- `append_column` rejects a wrong length or a duplicate name, and `rename_column` rejects a collision. `insert_column` and `replace_column` also check frames that have columns but no rows.
- Float and Int32 columns round-trip through `AnyRow`; `from_rows` widens mixed Int and Double columns to Double and rejects other mixtures.
- `Filter::negate` works; the bridge expected a doubly nested payload and always failed to decode it.
- CSV and JSON input keeps interior NUL bytes, and bridge error messages escape interior NULs instead of being cut short.
- Paths that are not valid UTF-8 are rejected instead of being converted lossily.
- The coverage audits explain their conflicting symbol counts (641 and 484) and what their 100% means.
- NaN no longer matches `gt`, `gte`, `lt`, `lte` or `between` filters, and sorting puts nulls first and NaN after every number instead of producing an inconsistent order.
- NaN and infinite values cross the bridge in both directions as `"NaN"`, `"Infinity"` and `"-Infinity"`: `rows`, `column`, `any_column`, `column_slice` and `rows_json` no longer fail on frames that hold them, and `Column::doubles`, `Column::dates`, `AnyValue::Double` and `AnyValue::Date` accept them.
- `slice_rows`, `prefix_rows`, `suffix_rows` and `column_slice` treat bounds above `isize::MAX` as the end of the frame instead of returning nothing.
- `ColumnSlice::range`, `AnyColumn::slice` and `Column::slice` return an empty slice for a reversed range instead of panicking, and `ColumnSlice::range` and `ColumnSlice::distinct` no longer panic when `indices` is shorter than `values`.
- `DataFrame::column`, `AnyColumn::to_column` and `ColumnSlice::to_column` read Float and Int32 columns, such as CSV columns read with `CSVType::Float`, instead of failing or turning them into Double columns.
- `mask_rows`, `filtered_by_column`, `insert_column`, `replace_column`, `remove_column`, `transform_column`, `transform_non_null_column`, `combine_columns2`, `combine_columns3`, `explode_column` and `exploding_column` run on `TabularData`'s own mask, insert, replace, remove and explode operations. They used to rebuild the whole frame in Rust from rows or typed columns, which turned Float and Int32 columns into Double and Int columns, failed on frames holding array, object or other columns that `Column` cannot represent, and dropped every alias. Column types and aliases now survive, including when the column is addressed through an alias.
- `insert_column` and `replace_column` reject a name that is already taken by a column or an alias instead of letting `TabularData` rename the column to `<name>.1` or silently move the alias, and `remove_column` through an alias removes that alias instead of leaving it dangling.
- `rename_column` through an alias renames the column by its real name, so the alias follows the new name instead of keeping the old one. `rename_column`, `append_column`, `insert_column` and `replace_column` also reject a name that is an alias of several columns, which `TabularData` would silently drop. Renaming or replacing a column with the name of its own alias is allowed and drops that alias, as in `TabularData`.
- `sorted_by`, `sort_by`, and `random_split` and `stratified_split` with a proportion of 0 or 1 keep the frame's aliases.
- `from_rows` accepts array and object values and creates array and object columns, so `DataFrame::from_rows(&frame.rows()?)` works for frames read from JSON.
- `GroupBy::random_split` without a seed picks a random one, like `DataFrame::random_split` and `TabularData`, instead of always using the same fixed seed and returning the same split.
- Null elements inside array and object cells read back as null instead of the string `"nil"` in `rows`, `row`, `any_column`, `column_slice` and `rows_json`, so a `transform_column` or `from_rows` round trip no longer turns them into strings.
- New `validation_tests` cover the error paths.

### Changed

- `sorted_by` converts each key column once and rebuilds the frame with `append(row:)`; `insert_row` and `replace_row` insert one row instead of rebuilding the frame; `append_rows_of` uses `append(rowsOf:)` when the schemas match; `json_bytes` returns the bytes directly instead of a JSON array of integers; CSV and JSON input is passed as bytes instead of three string copies.
- **Breaking:** validation failures report `TabularDataError::InvalidArgument`; several bridge errors that used to report `FrameworkError` now report `InvalidArgument`.
- **Breaking:** raw FFI: `td_dataframe_from_csv_data` and `td_dataframe_from_json_data` take a byte pointer and length, `td_dataframe_json_data_json` is replaced by `td_dataframe_json_data` (which returns a status and writes the buffer and its length through out-pointers), and `td_dataframe_append_rows_of` is new. `td_dataframe_column_json`, `td_dataframe_any_column_json`, `td_dataframe_column_slice_json`, `td_dataframe_column_mask_json` and `td_dataframe_row_json` return a status and write their payload through an out-pointer, and `td_dataframe_row_json` reads its index as unsigned.
- **Breaking:** `AnyValue::Double` and `AnyValue::Date` serialize NaN and infinite values as the strings `"NaN"`, `"Infinity"` and `"-Infinity"` instead of `null`, and deserialize them back; `rows_json` reports them the same way.
- **Breaking:** `column`, `any_column`, `column_slice`, `column_mask` and `row` report an unknown column name, an out-of-range row index or a mask of the wrong length as `InvalidArgument` instead of `FrameworkError`, like every other validation failure.
- **Breaking:** `remove_column` returns the removed column as an `AnyColumn`, which can hold every element type, instead of a `Column`.
- **Breaking:** `transform_column` and `transform_non_null_column` keep the column's element type and convert the closure's values with the same rules as `append_row` (for example 2.0 into an Int column, but no longer a String into a Data column); a value that does not convert returns `InvalidArgument` and leaves the column unchanged.
- **Breaking:** `explode_column` and `exploding_column` follow `TabularData`: rows whose array is empty or null are dropped instead of kept with a null. The exploded column takes the arrays' element type (String, Int, Double, Bool, Date, Data, array or object, with Int and Double widened to Double) instead of going through `from_rows`; arrays that mix other kinds, and columns that do not hold arrays, return `InvalidArgument` instead of being left unchanged.
- **Breaking:** `ColumnData` is `#[non_exhaustive]` and gains `Int32s` and `Floats`, built with `Column::int32s` and `Column::floats`. `Column::from_any_values`, `Column::with_capacity` and `ColumnData::with_capacity` map `Float` to `Floats` instead of `Doubles`, and `Int32` to `Int32s` instead of an error or a String column.
- **Breaking:** `ColumnData::with_capacity`, `Column::with_capacity` and `AnyColumnPrototype::make_column` return a `Result` and reject type names they cannot build, such as the `Array<Optional<Any>>` of an array column's prototype, instead of silently returning a String column.
- Requires `apple-cf` 0.11; `rust-version` is 1.82.

### Added

- Raw FFI: `td_dataframe_mask_rows`, `td_dataframe_insert_column_json`, `td_dataframe_replace_column_json`, `td_dataframe_remove_column_json`, `td_dataframe_transform_column_json` and `td_dataframe_exploding_column`.
- `frame_operation_tests` checks every frame operation above against Int, Int32, Float, Double, String, Bool, Date, Data, array and object columns and against aliases.

## [0.2.6] - 2026-05-18

- Added `///` docs across the public Rust wrapper surface, referencing the matching `TabularData` counterparts throughout `src/` (excluding `src/ffi/`).
- Raised rustdoc item coverage from 0.3% to 84.2% while keeping the existing API surface unchanged.

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
