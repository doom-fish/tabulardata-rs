# tabulardata-rs v0.2.2 coverage

This document tracks the public Rust API shipped in `v0.2.2`, the matching Swift bridge area, and the validation artifacts that exercise it.

Status meanings:

- ✅ implemented and covered by at least one example and one test

## Logical areas

| Area | Rust surface | Swift / SDK family | Example | Test | Status |
| --- | --- | --- | --- | --- | --- |
| DataFrame core | `DataFrame::{new, from_columns, shape, row_count, column_count, column_names, index_of_column, contains_column, contains_column_of_type, column_names_for_alias, add_alias, remove_alias, column, rows_json, description, format}` | `DataFrame`, `DataFrameProtocol` core frame/alias/description members | `01_smoke` | `dataframe_tests` | ✅ |
| ColumnSlice | `DataFrame::{column_slice, column_mask}`, `ColumnSlice::{len, is_empty, value, is_nil, range, distinct, summary, min, max, argmin, argmax, sum, mean, standard_deviation, description, to_column}` | `ColumnSlice`, `DiscontiguousColumnSlice`, `AnyColumnSlice` | `02_column_slice_summary` | `column_slice_tests` | ✅ |
| ColumnEncoder | `DataFrame::{encode_column, decode_column}`, `ColumnCodec`, `ColumnElementType` | column encoding / decoding family | `03_column_encoder_roundtrip` | `column_encoder_tests` | ✅ |
| Filter | `Filter::*`, `DataFrame::{filtered, filtered_by_column, mask_rows}` | `DataFrame.filter`, typed column filter overloads | `04_filter_rows` | `filter_tests` | ✅ |
| GroupBy | `DataFrame::{group_by, group_by_time}`, `GroupBy::{counts, sums, means, quantiles, minimums, maximums, group, group_count, ungrouped}` | `RowGroupingProtocol` aggregate family | `05_groupby_counts` | `groupby_tests` | ✅ |
| Join | `JoinColumns`, `DataFrame::{joined, joined_on}` | join family | `06_join_frames` | `join_tests` | ✅ |
| AnyColumn | `AnyValue`, `AnyColumn::{len, is_empty, value, is_nil, slice, distinct, summary, to_column, min, max, argmin, argmax, sum, mean, standard_deviation, description}`, `DataFrame::{any_column, any_columns}` | `AnyColumn`, `AnyColumnProtocol` | `07_any_column_snapshot` | `any_column_tests` | ✅ |
| Sort | `SortKey`, `SortOrder`, `DataFrame::{sorted_by, sort_by}` | sort family | `08_sort_rows` | `sort_tests` | ✅ |
| CSVReader | `CSVReadRequest`, `CSVType`, `DateParseStrategy`, `DataFrame::{from_csv, read_csv_with, from_csv_data, read_csv_data_with}` | CSV file/data readers | `09_csv_reader_subset`, `18_io_expansion` | `csv_reader_tests`, `io_expansion_tests` | ✅ |
| CSVWriter | `CSVWritingOptions`, `DateWriteStrategy`, `DataFrame::{write_csv, csv_string}` | CSV writer family | `10_csv_writer_string` | `csv_writer_tests` | ✅ |
| AnyRow | `AnyRow`, `DataFrame::{from_rows, row, rows, append_row, insert_row, replace_row, append_empty_row, remove_row, append_values, append_rows_of, append_frame}` | `DataFrame.Row`, `DataFrame.Rows`, row mutation family | `11_any_row_mutation` | `any_row_tests` | ✅ |
| Summary | `DataFrame::{summary_frame, summary_columns, summary_indices, column_summary}`, `NumericSummary`, `CategoricalSummary`, `ColumnSummary` | summary frame/value family | `12_summary_report` | `summary_tests` | ✅ |
| Slicing | `DataFrame::{slice_rows, prefix_rows, suffix_rows, select_columns}` | slicing / slice materialization family | `13_slicing_rows_and_columns` | `slicing_tests` | ✅ |
| JSON | `JSONReadRequest`, `JSONReadingOptions`, `JSONWritingOptions`, `JSONType`, `JSONReadingError`, `DataFrame::{from_json, read_json_with, from_json_data, read_json_data_with, from_json_string, read_json_string_with, write_json, json_bytes, json_string}` | JSON reader/writer family | `14_json_io` | `json_tests`, `io_expansion_tests` | ✅ |
| Typed protocols | `Column::{with_capacity, strings, ints, doubles, bools, dates, binary, from_any_values, value, values, slice, distinct, summary, min, max, argmin, argmax, sum, mean, standard_deviation, description}`, `ColumnId`, `ColumnPrototype`, `AnyColumnProtocol`, `OptionalColumnProtocol`, `ColumnProtocol`, `AnyColumnPrototype`, `DataFrameProtocol`, `DataFrameSlice`, `DataFrameRow`, `DataFrameRows`, `FilledColumn`, `SummaryColumnIds` | `ColumnProtocol`, `OptionalColumnProtocol`, `AnyColumnProtocol`, `AnyColumnPrototype`, `DataFrameProtocol`, `ColumnID`, typed column hash/description/typealias family | `15_typed_protocols` | `typed_protocol_tests` | ✅ |
| DataFrame mutation | `DataFrame::{try_clone, column_at, any_column_at, contains_column_id, column_by_id, insert_column, replace_column, remove_column, transform_column, transform_non_null_column, combine_columns2, combine_columns3, explode_column, exploding_column}` | DataFrame mutation / reshape family | `16_dataframe_mutation` | `dataframe_mutation_tests` | ✅ |
| Group summaries + group splits | `GroupBy::{filter_groups, map_groups, random_split, summary, summary_of}`, `GroupSummaries` | `GroupSummaries`, advanced `RowGroupingProtocol` filtering/split/summary members | `17_group_summary_split` | `group_summary_split_tests` | ✅ |
| SFrame + shaped data | `SFrameReadRequest`, `SFrameReadingError`, `DataFrame::{from_sframe, read_sframe_with}`, `ShapedData<T>` | SFrame import family, shaped export family | `18_io_expansion`, `15_typed_protocols` | `io_expansion_tests`, `typed_protocol_tests` | ✅ |

## Supporting public Rust types

| Type / API | Status | Notes |
| --- | --- | --- |
| `ColumnData::{Strings, Ints, Doubles, Bools, Dates, Data}` | ✅ | Typed column construction and schema-preserving mutation. |
| Prelude re-exports (`tabulardata::prelude::*`) | ✅ | Includes the expanded protocol, SFrame, and shaped-data surface. |
| `TabularDataError` | ✅ | Shared base error surface used by all bridged operations. |
| `JoinKind`, `GroupValueType`, `TimeUnit`, `CSVType`, `JSONType`, `ColumnCodec`, `ColumnElementType` | ✅ | Enum families exposed directly in Rust. |

## Validation run for this matrix

The v0.2.2 matrix above was validated with:

- `cargo test -- --test-threads=1`
- `cargo clippy --all-targets -- -D warnings`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
