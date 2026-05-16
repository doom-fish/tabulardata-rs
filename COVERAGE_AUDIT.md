# tabulardata-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 641
VERIFIED: 133
GAPS: 506
EXEMPT: 2
COVERAGE_PCT: 20.81%

Counts include public `init` and `subscript` declarations because TabularData exposes major functionality through constructors and collection accessors in `TabularData.swiftinterface`.

v0.2.1 focus: JSON IO, split/formatting, and DataFrame metadata/alias/row-mutation helpers are now verified. Remaining gaps are still dominated by Swift protocol-only typed column hierarchy, `ColumnID` overloads, `GroupSummaries`/summary value types, dedicated JSON/CSV error enums, and `ShapedData`.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `DataFrame.mutating func renameColumn(_ name: Swift.String, to newName: Swift.String)` | func | `TabularData.swiftinterface:L32` | `DataFrame::rename_column` |
| `DataFrame` | struct | `TabularData.swiftinterface:L43` | `tabulardata::DataFrame` |
| `DataFrame.var columns: [TabularData.AnyColumn]` | var | `TabularData.swiftinterface:L44` | `DataFrame::{column_names,select_columns,summary_indices,any_columns}` |
| `DataFrame.var rows: TabularData.DataFrame.Rows` | var | `TabularData.swiftinterface:L47` | `DataFrame::{rows_json,row,rows}` |
| `DataFrame.var shape: (rows: Swift.Int, columns: Swift.Int)` | var | `TabularData.swiftinterface:L51` | `DataFrame::{shape,row_count,column_count}` |
| `DataFrame.init()` | init | `TabularData.swiftinterface:L54` | `DataFrame::new` |
| `DataFrame.init<S>(columns: S) where S : Swift.Sequence, S.Element == TabularData.AnyColumn` | init | `TabularData.swiftinterface:L55` | `DataFrame::from_columns` |
| `DataFrame.init(_ other: TabularData.DataFrame.Slice)` | init | `TabularData.swiftinterface:L56` | `DataFrame::{slice_rows,prefix_rows,suffix_rows,filtered,group}` |
| `DataFrame.mutating func append(column: TabularData.AnyColumn)` | func | `TabularData.swiftinterface:L82` | `DataFrame::{from_columns,append_column}` |
| `DataFrame.mutating func append(valuesByColumn dictionary: [Swift.String : Any?])` | func | `TabularData.swiftinterface:L106` | `DataFrame::{from_rows,append_row}` |
| `DataFrame.func prefix(_ maxLength: Swift.Int) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L111` | `DataFrame::prefix_rows` |
| `DataFrame.func suffix(_ maxLength: Swift.Int) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L112` | `DataFrame::suffix_rows` |
| `DataFrame.func filter(_ isIncluded: (TabularData.DataFrame.Row) throws -> Swift.Bool) rethrows -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L122` | `DataFrame::filtered` |
| `DataFrame.func selecting<S>(columnNames: S) -> TabularData.DataFrame where S : Swift.Sequence, S.Element == Swift.String` | func | `TabularData.swiftinterface:L123` | `DataFrame::{select_columns,summary_columns}` |
| `DataFrame.subscript(columnName: Swift.String) -> TabularData.AnyColumn` | subscript | `TabularData.swiftinterface:L135` | `DataFrame::{column,any_column,column_slice,column_mask}` |
| `DataFrame.subscript(row index: Swift.Int) -> TabularData.DataFrame.Row` | subscript | `TabularData.swiftinterface:L213` | `DataFrame::{row,replace_row}` |
| `DataFrameProtocol.func writeCSV(to url: Foundation.URL, options: TabularData.CSVWritingOptions = .init()) throws` | func | `TabularData.swiftinterface:L408` | `DataFrame::write_csv` |
| `DataFrameProtocol.func csvRepresentation(options: TabularData.CSVWritingOptions = .init()) throws -> Foundation.Data` | func | `TabularData.swiftinterface:L409` | `DataFrame::csv_string` |
| `DataFrame.Slice` | struct | `TabularData.swiftinterface:L413` | `DataFrame::{slice_rows,prefix_rows,suffix_rows,filtered,group}` |
| `Column` | struct | `TabularData.swiftinterface:L474` | `tabulardata::Column` |
| `Column.init<S>(name: Swift.String, contents: S) where S : Swift.Sequence, S.Element == WrappedElement?` | init | `TabularData.swiftinterface:L493` | `Column::{strings,ints,doubles,bools}` |
| `Column.init<S>(name: Swift.String, contents: S) where WrappedElement == S.Element, S : Swift.Sequence` | init | `TabularData.swiftinterface:L495` | `Column::{strings,ints,doubles,bools}` |
| `Column.func eraseToAnyColumn() -> TabularData.AnyColumn` | func | `TabularData.swiftinterface:L523` | `DataFrame::{from_columns,append_column}` |
| `DataFrame.func summary() -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1165` | `DataFrame::{summary,summary_frame}` |
| `DataFrame.func summary(of columnNames: Swift.String...) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1166` | `DataFrame::summary_columns` |
| `DataFrame.func summary(ofColumns columnIndices: Swift.Int...) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1167` | `DataFrame::summary_indices` |
| `DataFrame.init(contentsOfCSVFile url: Foundation.URL, columns: [Swift.String]? = nil, rows: Swift.Range<Swift.Int>? = nil, types: [Swift.String : TabularData.CSVType] = [:], options: TabularData.CSVReadingOptions = .init()) throws` | init | `TabularData.swiftinterface:L1217` | `DataFrame::{from_csv,read_csv_with}` |
| `DataFrameProtocol.subscript(range: Swift.Range<Swift.Int>) -> TabularData.DataFrame.Slice` | subscript | `TabularData.swiftinterface:L1238` | `DataFrame::slice_rows` |
| `DataFrameProtocol.subscript(range: Swift.Range<Swift.Int>) -> TabularData.DataFrame.Slice` | subscript | `TabularData.swiftinterface:L1251` | `DataFrame::slice_rows` |
| `DataFrame.mutating func decode<T, Decoder>(_ type: T.Type, inColumn columnName: Swift.String, using decoder: Decoder) throws where T : Swift.Decodable, Decoder : Combine.TopLevelDecoder` | func | `TabularData.swiftinterface:L1289` | `DataFrame::decode_column` |
| `DataFrame.mutating func encodeColumn<T, Encoder>(_ columnName: Swift.String, _ type: T.Type, using encoder: Encoder) throws where T : Swift.Encodable, Encoder : Combine.TopLevelEncoder` | func | `TabularData.swiftinterface:L1679` | `DataFrame::encode_column` |
| `RowGroupingProtocol` | protocol | `TabularData.swiftinterface:L1771` | `GroupBy` |
| `RowGroupingProtocol.func counts(order: TabularData.Order?) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1775` | `GroupBy::counts` |
| `RowGroupingProtocol.subscript(keys: Any?...) -> TabularData.DataFrame.Slice?` | subscript | `TabularData.swiftinterface:L1789` | `GroupBy::group` |
| `RowGroupingProtocol.func counts() -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1794` | `GroupBy::counts` |
| `RowGroupingProtocol.func sums<N>(_ columnName: Swift.String, _ type: N.Type, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.AdditiveArithmetic, N : Swift.Comparable` | func | `TabularData.swiftinterface:L1796` | `GroupBy::sums` |
| `RowGroupingProtocol.func means<N>(_ columnName: Swift.String, _ type: N.Type, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.FloatingPoint` | func | `TabularData.swiftinterface:L1802` | `GroupBy::means` |
| `RowGroupingProtocol.func quantiles<N>(_ columnName: Swift.String, _ type: N.Type, quantile: N, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.BinaryFloatingPoint` | func | `TabularData.swiftinterface:L1808` | `GroupBy::quantiles` |
| `RowGroupingProtocol.func minimums<N>(_ columnName: Swift.String, _ type: N.Type, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.Comparable` | func | `TabularData.swiftinterface:L1814` | `GroupBy::minimums` |
| `RowGroupingProtocol.func maximums<N>(_ columnName: Swift.String, _ type: N.Type, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.Comparable` | func | `TabularData.swiftinterface:L1820` | `GroupBy::maximums` |
| `Order` | enum | `TabularData.swiftinterface:L1834` | `SortOrder and GroupBy order parameters` |
| `CSVReadingOptions` | struct | `TabularData.swiftinterface:L1927` | `tabulardata::CSVReadingOptions` |
| `CSVReadingOptions.var hasHeaderRow: Swift.Bool` | var | `TabularData.swiftinterface:L1928` | `CSVReadingOptions::with_has_header_row` |
| `CSVReadingOptions.var nilEncodings: Swift.Set<Swift.String>` | var | `TabularData.swiftinterface:L1929` | `CSVReadingOptions::with_nil_encodings` |
| `CSVReadingOptions.var trueEncodings: Swift.Set<Swift.String>` | var | `TabularData.swiftinterface:L1930` | `CSVReadingOptions::with_true_encodings` |
| `CSVReadingOptions.var falseEncodings: Swift.Set<Swift.String>` | var | `TabularData.swiftinterface:L1931` | `CSVReadingOptions::with_false_encodings` |
| `CSVReadingOptions.var floatingPointType: TabularData.CSVType` | var | `TabularData.swiftinterface:L1932` | `CSVReadingOptions::with_floating_point_type` |
| `CSVReadingOptions.var dateParsers: [(Swift.String) -> Foundation.Date?]` | var | `TabularData.swiftinterface:L1933` | `CSVReadingOptions::with_date_parse_strategy` |
| `CSVReadingOptions.var ignoresEmptyLines: Swift.Bool` | var | `TabularData.swiftinterface:L1934` | `CSVReadingOptions::with_ignores_empty_lines` |
| `CSVReadingOptions.var usesQuoting: Swift.Bool` | var | `TabularData.swiftinterface:L1935` | `CSVReadingOptions::with_uses_quoting` |
| `CSVReadingOptions.var usesEscaping: Swift.Bool` | var | `TabularData.swiftinterface:L1936` | `CSVReadingOptions::with_uses_escaping` |
| `CSVReadingOptions.var delimiter: Swift.Character` | var | `TabularData.swiftinterface:L1937` | `CSVReadingOptions::with_delimiter` |
| `CSVReadingOptions.var escapeCharacter: Swift.Character` | var | `TabularData.swiftinterface:L1940` | `CSVReadingOptions::with_escape_character` |
| `CSVReadingOptions.init(hasHeaderRow: Swift.Bool = true, nilEncodings: Swift.Set<Swift.String> = ["", "#N/A", "#N/A N/A", "#NA", "N/A", "NA", "NULL", "n/a", "nil", "null"], trueEncodings: Swift.Set<Swift.String> = ["1", "True", "TRUE", "true"], falseEncodings: Swift.Set<Swift.String> = ["0", "False", "FALSE", "false"], floatingPointType: TabularData.CSVType = .double, ignoresEmptyLines: Swift.Bool = true, usesQuoting: Swift.Bool = true, usesEscaping: Swift.Bool = false, delimiter: Swift.Character = Character(","), escapeCharacter: Swift.Character = Character("\\"))` | init | `TabularData.swiftinterface:L1943` | `CSVReadingOptions::new` |
| `CSVReadingOptions.mutating func addDateParseStrategy<T>(_ strategy: T) where T : Foundation.ParseStrategy, T.ParseInput == Swift.String, T.ParseOutput == Foundation.Date` | func | `TabularData.swiftinterface:L1944` | `CSVReadingOptions::with_date_parse_strategy` |
| `CSVWritingOptions` | struct | `TabularData.swiftinterface:L1947` | `tabulardata::CSVWritingOptions` |
| `CSVWritingOptions.var includesHeader: Swift.Bool` | var | `TabularData.swiftinterface:L1948` | `CSVWritingOptions::with_includes_header` |
| `CSVWritingOptions.var dateFormatter: (Foundation.Date) -> Swift.String` | var | `TabularData.swiftinterface:L1952` | `CSVWritingOptions::with_date_strategy` |
| `CSVWritingOptions.var nilEncoding: Swift.String` | var | `TabularData.swiftinterface:L1956` | `CSVWritingOptions::with_nil_encoding` |
| `CSVWritingOptions.var trueEncoding: Swift.String` | var | `TabularData.swiftinterface:L1957` | `CSVWritingOptions::with_true_encoding` |
| `CSVWritingOptions.var falseEncoding: Swift.String` | var | `TabularData.swiftinterface:L1958` | `CSVWritingOptions::with_false_encoding` |
| `CSVWritingOptions.var newline: Swift.String` | var | `TabularData.swiftinterface:L1959` | `CSVWritingOptions::with_newline` |
| `CSVWritingOptions.var delimiter: Swift.Character` | var | `TabularData.swiftinterface:L1960` | `CSVWritingOptions::with_delimiter` |
| `CSVWritingOptions.init()` | init | `TabularData.swiftinterface:L1961` | `CSVWritingOptions::new` |
| `RowGrouping.subscript(keys: Any?...) -> TabularData.DataFrame.Slice?` | subscript | `TabularData.swiftinterface:L2026` | `GroupBy::group` |
| `DataFrame.Row` | struct | `TabularData.swiftinterface:L2100` | `AnyRow and DataFrame::{row,rows}` |
| `CSVType` | enum | `TabularData.swiftinterface:L2324` | `tabulardata::CSVType` |
| `DataFrame.func grouped(by columnName: Swift.String) -> any TabularData.RowGroupingProtocol` | func | `TabularData.swiftinterface:L2340` | `DataFrame::group_by` |
| `DataFrameProtocol.func grouped(by columnName: Swift.String, timeUnit: Foundation.Calendar.Component) -> TabularData.RowGrouping<Swift.Int>` | func | `TabularData.swiftinterface:L2355` | `DataFrame::group_by_time` |
| `DataFrameProtocol.func grouped(by columnNames: Swift.String...) -> some TabularData.RowGroupingProtocol` | func | `TabularData.swiftinterface:L2357` | `DataFrame::group_by` |
| `JoinKind` | enum | `TabularData.swiftinterface:L2467` | `tabulardata::JoinKind` |
| `DataFrameProtocol.func joined<R>(_ other: R, on columnName: Swift.String, kind: TabularData.JoinKind = .inner) -> TabularData.DataFrame where R : TabularData.DataFrameProtocol` | func | `TabularData.swiftinterface:L2480` | `DataFrame::joined` |
| `DataFrameProtocol.func joined<R>(_ other: R, on columnNames: (left: Swift.String, right: Swift.String), kind: TabularData.JoinKind = .inner) -> TabularData.DataFrame where R : TabularData.DataFrameProtocol` | func | `TabularData.swiftinterface:L2482` | `DataFrame::joined_on` |
| `AnyColumn` | struct | `TabularData.swiftinterface:L2486` | `tabulardata::AnyColumn` |
| `AnyColumn.var name: Swift.String` | var | `TabularData.swiftinterface:L2487` | `DataFrame::any_column` |
| `AnyColumn.var wrappedElementType: any Any.Type` | var | `TabularData.swiftinterface:L2491` | `AnyColumn.type_name` |
| `AnyColumn.var count: Swift.Int` | var | `TabularData.swiftinterface:L2497` | `AnyColumn::len` |
| `AnyColumn.var missingCount: Swift.Int` | var | `TabularData.swiftinterface:L2501` | `AnyColumn.missing_count` |
| `AnyColumn.subscript(range: Swift.Range<Swift.Int>) -> TabularData.AnyColumnSlice` | subscript | `TabularData.swiftinterface:L2532` | `DataFrame::column_slice` |
| `AnyColumn.subscript<C>(mask: C) -> TabularData.AnyColumnSlice where C : Swift.Collection, C.Element == Swift.Bool` | subscript | `TabularData.swiftinterface:L2536` | `DataFrame::column_mask` |
| `DataFrame.func indexOfColumn(_ columnName: Swift.String) -> Swift.Int?` | func | `TabularData.swiftinterface:L58` | DataFrame::index_of_column |
| `DataFrame.func containsColumn<T>(_ name: Swift.String, _ type: T.Type) -> Swift.Bool` | func | `TabularData.swiftinterface:L68` | DataFrame::contains_column_of_type |
| `DataFrame.func containsColumn(_ name: Swift.String) -> Swift.Bool` | func | `TabularData.swiftinterface:L75` | DataFrame::contains_column |
| `DataFrame.func columnNames(forAlias alias: Swift.String) -> [Swift.String]` | func | `TabularData.swiftinterface:L78` | DataFrame::column_names_for_alias |
| `DataFrame.mutating func addAlias(_ alias: Swift.String, forColumn columnName: Swift.String)` | func | `TabularData.swiftinterface:L79` | DataFrame::add_alias |
| `DataFrame.mutating func removeAlias(_ alias: Swift.String)` | func | `TabularData.swiftinterface:L80` | DataFrame::remove_alias |
| `DataFrame.mutating func append<T>(column: TabularData.Column<T>)` | func | `TabularData.swiftinterface:L81` | DataFrame::append_column |
| `DataFrame.mutating func append(row: TabularData.DataFrame.Row)` | func | `TabularData.swiftinterface:L101` | DataFrame::append_row |
| `DataFrame.mutating func appendEmptyRow()` | func | `TabularData.swiftinterface:L108` | DataFrame::append_empty_row |
| `DataFrame.mutating func insert(row: TabularData.DataFrame.Row, at index: Swift.Int)` | func | `TabularData.swiftinterface:L109` | DataFrame::insert_row |
| `DataFrame.mutating func removeRow(at index: Swift.Int)` | func | `TabularData.swiftinterface:L110` | DataFrame::remove_row |
| `DataFrameProtocol.func randomSplit(by proportion: Swift.Double, seed: Swift.Int? = nil) -> (TabularData.DataFrame.Slice, TabularData.DataFrame.Slice)` | func | `TabularData.swiftinterface:L247` | DataFrame::random_split |
| `DataFrameProtocol.func stratifiedSplit(on columnName: Swift.String, by proportion: Swift.Double, randomSeed: Swift.Int? = nil) -> (TabularData.DataFrame, TabularData.DataFrame)` | func | `TabularData.swiftinterface:L251` | DataFrame::stratified_split |
| `DataFrameProtocol.func stratifiedSplit(on columnNames: Swift.String..., by proportion: Swift.Double, randomSeed: Swift.Int? = nil) -> (TabularData.DataFrame, TabularData.DataFrame)` | func | `TabularData.swiftinterface:L254` | DataFrame::stratified_split |
| `DiscontiguousColumnSlice.func summary() -> TabularData.CategoricalSummary<WrappedElement>` | func | `TabularData.swiftinterface:L272` | ColumnSlice::summary |
| `ColumnSlice.func summary() -> TabularData.CategoricalSummary<WrappedElement>` | func | `TabularData.swiftinterface:L276` | ColumnSlice::summary |
| `AnyColumnSlice.func summary() -> TabularData.AnyCategoricalSummary` | func | `TabularData.swiftinterface:L284` | ColumnSlice::summary |
| `JSONType` | enum | `TabularData.swiftinterface:L307` | tabulardata::JSONType |
| `JSONReadingOptions` | struct | `TabularData.swiftinterface:L379` | tabulardata::JSONReadingOptions |
| `JSONReadingOptions.var dateParsers: [(Swift.String) -> Foundation.Date?]` | var | `TabularData.swiftinterface:L380` | JSONReadingOptions::with_date_parse_strategy |
| `JSONReadingOptions.init()` | init | `TabularData.swiftinterface:L381` | JSONReadingOptions::new |
| `JSONReadingOptions.mutating func addDateParseStrategy<T>(_ strategy: T) where T : Foundation.ParseStrategy, T.ParseInput == Swift.String, T.ParseOutput == Foundation.Date` | func | `TabularData.swiftinterface:L382` | JSONReadingOptions::with_date_parse_strategy |
| `JSONWritingOptions` | struct | `TabularData.swiftinterface:L385` | tabulardata::JSONWritingOptions |
| `JSONWritingOptions.var sortKeys: Swift.Bool` | var | `TabularData.swiftinterface:L386` | JSONWritingOptions::with_sort_keys |
| `JSONWritingOptions.var prettyPrint: Swift.Bool` | var | `TabularData.swiftinterface:L387` | JSONWritingOptions::with_pretty_print |
| `JSONWritingOptions.var dateFormatter: (Foundation.Date) -> Swift.String` | var | `TabularData.swiftinterface:L388` | JSONWritingOptions::with_date_strategy |
| `JSONWritingOptions.init()` | init | `TabularData.swiftinterface:L389` | JSONWritingOptions::new |
| `Column.var name: Swift.String` | var | `TabularData.swiftinterface:L476` | Column::name |
| `Column.var count: Swift.Int` | var | `TabularData.swiftinterface:L477` | Column::len |
| `Column.var missingCount: Swift.Int` | var | `TabularData.swiftinterface:L481` | Column::missing_count |
| `Column.var wrappedElementType: any Any.Type` | var | `TabularData.swiftinterface:L484` | Column::type_name |
| `DataFrameProtocol.func sorted(on columnName: Swift.String, order: TabularData.Order = .ascending) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1312` | DataFrame::sorted_by |
| `DataFrameProtocol.func sorted<T>(on columnName: Swift.String, _ type: T.Type, order: TabularData.Order = .ascending) -> TabularData.DataFrame where T : Swift.Comparable` | func | `TabularData.swiftinterface:L1313` | DataFrame::sorted_by |
| `DataFrame.mutating func sort(on columnName: Swift.String, order: TabularData.Order = .ascending)` | func | `TabularData.swiftinterface:L1322` | DataFrame::sort_by |
| `DataFrame.mutating func sort<T>(on columnName: Swift.String, _ type: T.Type, order: TabularData.Order = .ascending) where T : Swift.Comparable` | func | `TabularData.swiftinterface:L1323` | DataFrame::sort_by |
| `DataFrame.var description: Swift.String` | var | `TabularData.swiftinterface:L1376` | DataFrame::description |
| `DataFrameProtocol.func description(options: TabularData.FormattingOptions) -> Swift.String` | func | `TabularData.swiftinterface:L1413` | DataFrame::format |
| `DataFrame.init(contentsOfJSONFile url: Foundation.URL, columns: [Swift.String]? = nil, types: [Swift.String : TabularData.JSONType] = [:], options: TabularData.JSONReadingOptions = .init()) throws` | init | `TabularData.swiftinterface:L1662` | DataFrame::{from_json,read_json_with} |
| `DataFrame.init(jsonData data: Foundation.Data, columns: [Swift.String]? = nil, types: [Swift.String : TabularData.JSONType] = [:], options: TabularData.JSONReadingOptions = .init()) throws` | init | `TabularData.swiftinterface:L1665` | DataFrame::{from_json_data,read_json_data_with} |
| `DataFrameProtocol.func writeJSON(to url: Foundation.URL, options: TabularData.JSONWritingOptions = .init()) throws` | func | `TabularData.swiftinterface:L2320` | DataFrame::write_json |
| `DataFrameProtocol.func jsonRepresentation(options: TabularData.JSONWritingOptions = .init()) throws -> Foundation.Data` | func | `TabularData.swiftinterface:L2321` | DataFrame::{json_bytes,json_string} |
| `AnyColumn.func distinct() -> TabularData.AnyColumnSlice` | func | `TabularData.swiftinterface:L2566` | AnyColumn::distinct |
| `FormattingOptions` | struct | `TabularData.swiftinterface:L2601` | tabulardata::FormattingOptions |
| `FormattingOptions.var maximumLineWidth: Swift.Int` | var | `TabularData.swiftinterface:L2602` | FormattingOptions::with_maximum_line_width |
| `FormattingOptions.var maximumCellWidth: Swift.Int` | var | `TabularData.swiftinterface:L2603` | FormattingOptions::with_maximum_cell_width |
| `FormattingOptions.var maximumRowCount: Swift.Int` | var | `TabularData.swiftinterface:L2604` | FormattingOptions::with_maximum_row_count |
| `FormattingOptions.var includesColumnTypes: Swift.Bool` | var | `TabularData.swiftinterface:L2605` | FormattingOptions::with_includes_column_types |
| `FormattingOptions.var includesRowIndices: Swift.Bool` | var | `TabularData.swiftinterface:L2607` | FormattingOptions::with_includes_row_indices |
| `FormattingOptions.var includesRowAndColumnCounts: Swift.Bool` | var | `TabularData.swiftinterface:L2609` | FormattingOptions::with_includes_row_and_column_counts |
| `FormattingOptions.var locale: Foundation.Locale` | var | `TabularData.swiftinterface:L2626` | FormattingOptions::with_locale |
| `FormattingOptions.init()` | init | `TabularData.swiftinterface:L2630` | FormattingOptions::new |
| `FormattingOptions.init(locale: Foundation.Locale)` | init | `TabularData.swiftinterface:L2632` | FormattingOptions::{new,with_locale} |
| `FormattingOptions.init(maximumLineWidth: Swift.Int, maximumCellWidth: Swift.Int = 50, maximumRowCount: Swift.Int = 20, includesColumnTypes: Swift.Bool = true)` | init | `TabularData.swiftinterface:L2633` | FormattingOptions::{new,with_maximum_line_width,...} |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `ColumnProtocol` | protocol | `TabularData.swiftinterface:L13` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `ColumnProtocol.var name: Swift.String` | var | `TabularData.swiftinterface:L14` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `OptionalColumnProtocol` | protocol | `TabularData.swiftinterface:L17` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnProtocol` | protocol | `TabularData.swiftinterface:L21` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnProtocol.var name: Swift.String` | var | `TabularData.swiftinterface:L22` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnProtocol.var count: Swift.Int` | var | `TabularData.swiftinterface:L23` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnProtocol.var wrappedElementType: any Any.Type` | var | `TabularData.swiftinterface:L24` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnProtocol.subscript(position: Swift.Int) -> Any?` | subscript | `TabularData.swiftinterface:L26` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnProtocol.subscript(range: Swift.Range<Swift.Int>) -> TabularData.AnyColumnSlice` | subscript | `TabularData.swiftinterface:L28` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `DataFrame.mutating func replaceColumn(_ name: Swift.String, with newColumn: TabularData.AnyColumn)` | func | `TabularData.swiftinterface:L34` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func replaceColumn<T>(_ id: TabularData.ColumnID<T>, with newColumn: TabularData.AnyColumn)` | func | `TabularData.swiftinterface:L36` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func replaceColumn<T>(_ name: Swift.String, with newColumn: TabularData.Column<T>)` | func | `TabularData.swiftinterface:L38` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func replaceColumn<T, U>(_ id: TabularData.ColumnID<T>, with newColumn: TabularData.Column<U>)` | func | `TabularData.swiftinterface:L40` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.func containsColumn<T>(_ id: TabularData.ColumnID<T>) -> Swift.Bool` | func | `TabularData.swiftinterface:L61` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.mutating func insert<T>(column: TabularData.Column<T>, at index: Swift.Int)` | func | `TabularData.swiftinterface:L83` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.mutating func insert(column: TabularData.AnyColumn, at index: Swift.Int)` | func | `TabularData.swiftinterface:L84` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.mutating func removeColumn<T>(_ id: TabularData.ColumnID<T>) -> TabularData.Column<T>` | func | `TabularData.swiftinterface:L86` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func removeColumn(_ name: Swift.String) -> TabularData.AnyColumn` | func | `TabularData.swiftinterface:L88` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func transformColumn<From, To>(_ id: TabularData.ColumnID<From>, _ transform: (From?) throws -> To?) rethrows` | func | `TabularData.swiftinterface:L90` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func transformColumn<From, To>(_ id: TabularData.ColumnID<From>, _ transform: (From) throws -> To?) rethrows` | func | `TabularData.swiftinterface:L93` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func transformColumn<From, To>(_ name: Swift.String, _ transform: (From?) throws -> To?) rethrows` | func | `TabularData.swiftinterface:L96` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func transformColumn<From, To>(_ name: Swift.String, _ transform: (From) throws -> To?) rethrows` | func | `TabularData.swiftinterface:L99` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func append(row: Any?...)` | func | `TabularData.swiftinterface:L103` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.mutating func append(rowsOf other: TabularData.DataFrame)` | func | `TabularData.swiftinterface:L113` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.mutating func append(_ other: TabularData.DataFrame)` | func | `TabularData.swiftinterface:L114` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.mutating func append(_ other: TabularData.DataFrame.Slice)` | func | `TabularData.swiftinterface:L115` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.func filter<T>(on columnName: Swift.String, _ type: T.Type, _ isIncluded: (T?) throws -> Swift.Bool) rethrows -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L117` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.func filter<T>(on columnID: TabularData.ColumnID<T>, _ isIncluded: (T?) throws -> Swift.Bool) rethrows -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L120` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.func selecting(columnNames: Swift.String...) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L124` | The variadic selecting overload is not wrapped; Rust uses the sequence overload. |
| `DataFrame.typealias ColumnType = TabularData.AnyColumn` | typealias | `TabularData.swiftinterface:L126` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.subscript(dynamicMember columnName: Swift.String) -> TabularData.AnyColumn` | subscript | `TabularData.swiftinterface:L130` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.subscript<T>(columnName: Swift.String, type: T.Type = T.self) -> [T?]` | subscript | `TabularData.swiftinterface:L156` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.subscript<T>(columnName: Swift.String, type: T.Type) -> TabularData.Column<T>` | subscript | `TabularData.swiftinterface:L172` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.subscript<T>(id: TabularData.ColumnID<T>) -> TabularData.Column<T>` | subscript | `TabularData.swiftinterface:L187` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.subscript<S>(columnNames: S) -> TabularData.DataFrame where S : Swift.Sequence, S.Element == Swift.String` | subscript | `TabularData.swiftinterface:L202` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.subscript(column index: Swift.Int) -> TabularData.AnyColumn` | subscript | `TabularData.swiftinterface:L205` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.subscript<T>(column index: Swift.Int, type: T.Type) -> TabularData.Column<T>` | subscript | `TabularData.swiftinterface:L209` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.subscript<C>(mask: C) -> TabularData.DataFrame.Slice where C : Swift.Collection, C.Element == Swift.Bool` | subscript | `TabularData.swiftinterface:L217` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L224` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L225` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.init(dictionaryLiteral elements: (Swift.String, [Any?])...)` | init | `TabularData.swiftinterface:L232` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.typealias Key = Swift.String` | typealias | `TabularData.swiftinterface:L235` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.typealias Value = [Any?]` | typealias | `TabularData.swiftinterface:L237` | No public Rust wrapper for this SDK symbol. |
| `AnyColumnPrototype` | protocol | `TabularData.swiftinterface:L240` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnPrototype.var name: Swift.String` | var | `TabularData.swiftinterface:L241` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `AnyColumnPrototype.func makeColumn(capacity: Swift.Int) -> TabularData.AnyColumn` | func | `TabularData.swiftinterface:L242` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `DataFrameProtocol.func randomSplit<G>(by proportion: Swift.Double, using generator: inout G) -> (TabularData.DataFrame.Slice, TabularData.DataFrame.Slice) where G : Swift.RandomNumberGenerator` | func | `TabularData.swiftinterface:L249` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func stratifiedSplit<T>(on columnID: TabularData.ColumnID<T>, by proportion: Swift.Double, randomSeed: Swift.Int? = nil) -> (TabularData.DataFrame, TabularData.DataFrame) where T : Swift.Hashable` | func | `TabularData.swiftinterface:L257` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func stratifiedSplit<T0, T1>(on columnID0: TabularData.ColumnID<T0>, _ columnID1: TabularData.ColumnID<T1>, by proportion: Swift.Double, randomSeed: Swift.Int? = nil) -> (TabularData.DataFrame, TabularData.DataFrame) where T0 : Swift.Hashable, T1 : Swift.Hashable` | func | `TabularData.swiftinterface:L260` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func stratifiedSplit<T0, T1, T2>(on columnID0: TabularData.ColumnID<T0>, _ columnID1: TabularData.ColumnID<T1>, _ columnID2: TabularData.ColumnID<T2>, by proportion: Swift.Double, randomSeed: Swift.Int? = nil) -> (TabularData.DataFrame, TabularData.DataFrame) where T0 : Swift.Hashable, T1 : Swift.Hashable, T2 : Swift.Hashable` | func | `TabularData.swiftinterface:L263` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `Column.func summary() -> TabularData.CategoricalSummary<WrappedElement>` | func | `TabularData.swiftinterface:L268` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func summary() -> TabularData.CategoricalSummary<TabularData.FilledColumn<Base>.WrappedElement>` | func | `TabularData.swiftinterface:L280` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `JSONType.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L316` | JSON IO and JSON-specific errors are not wrapped. |
| `JSONType.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L317` | JSON IO and JSON-specific errors are not wrapped. |
| `SummaryColumnIDs` | enum | `TabularData.swiftinterface:L392` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.Slice.var base: TabularData.DataFrame` | var | `TabularData.swiftinterface:L414` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.var rows: TabularData.DataFrame.Rows` | var | `TabularData.swiftinterface:L417` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.var columns: [TabularData.AnyColumnSlice]` | var | `TabularData.swiftinterface:L421` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.var shape: (rows: Swift.Int, columns: Swift.Int)` | var | `TabularData.swiftinterface:L424` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.typealias ColumnType = TabularData.AnyColumnSlice` | typealias | `TabularData.swiftinterface:L428` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.subscript(dynamicMember columnName: Swift.String) -> TabularData.AnyColumnSlice` | subscript | `TabularData.swiftinterface:L433` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.subscript(columnName: Swift.String) -> TabularData.AnyColumnSlice` | subscript | `TabularData.swiftinterface:L436` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.subscript<T>(columnName: Swift.String, type: T.Type) -> TabularData.DiscontiguousColumnSlice<T>` | subscript | `TabularData.swiftinterface:L439` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.subscript<T>(columnID: TabularData.ColumnID<T>) -> TabularData.DiscontiguousColumnSlice<T>` | subscript | `TabularData.swiftinterface:L442` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.Slice.subscript<T>(column index: Swift.Int, type: T.Type) -> TabularData.DiscontiguousColumnSlice<T>` | subscript | `TabularData.swiftinterface:L445` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.subscript<S>(columnNames: S) -> TabularData.DataFrame.Slice where S : Swift.Sequence, S.Element == Swift.String` | subscript | `TabularData.swiftinterface:L448` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func filter<T>(on columnName: Swift.String, _ type: T.Type, _ isIncluded: (T?) throws -> Swift.Bool) rethrows -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L452` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func filter<T>(on columnID: TabularData.ColumnID<T>, _ isIncluded: (T?) throws -> Swift.Bool) rethrows -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L455` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.Slice.func prefix(_ length: Swift.Int) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L457` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func prefix(through position: Swift.Int) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L458` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func prefix(upTo position: Swift.Int) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L459` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func suffix(_ length: Swift.Int) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L460` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func suffix(from position: Swift.Int) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L461` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func selecting<S>(columnNames: S) -> TabularData.DataFrame.Slice where S : Swift.Sequence, S.Element == Swift.String` | func | `TabularData.swiftinterface:L462` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func selecting(columnNames: Swift.String...) -> TabularData.DataFrame.Slice` | func | `TabularData.swiftinterface:L463` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L468` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L469` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `Column.typealias Element = WrappedElement?` | typealias | `TabularData.swiftinterface:L475` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.var prototype: any TabularData.AnyColumnPrototype` | var | `TabularData.swiftinterface:L487` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.init(name: Swift.String, capacity: Swift.Int)` | init | `TabularData.swiftinterface:L490` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.init(_ id: TabularData.ColumnID<WrappedElement>, capacity: Swift.Int)` | init | `TabularData.swiftinterface:L491` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `Column.init<S>(_ id: TabularData.ColumnID<S.Element>, contents: S) where S : Swift.Sequence, S.Element == WrappedElement?` | init | `TabularData.swiftinterface:L497` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `Column.init<S>(_ id: TabularData.ColumnID<S.Element>, contents: S) where WrappedElement == S.Element, S : Swift.Sequence` | init | `TabularData.swiftinterface:L499` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `Column.init(_ slice: TabularData.ColumnSlice<WrappedElement>)` | init | `TabularData.swiftinterface:L500` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func append(_ element: TabularData.Column<WrappedElement>.Element)` | func | `TabularData.swiftinterface:L502` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func append(_ element: WrappedElement)` | func | `TabularData.swiftinterface:L504` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func append<S>(contentsOf sequence: S) where S : Swift.Sequence, S.Element == WrappedElement?` | func | `TabularData.swiftinterface:L506` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func append<S>(contentsOf sequence: S) where WrappedElement == S.Element, S : Swift.Sequence` | func | `TabularData.swiftinterface:L508` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func remove(at index: Swift.Int)` | func | `TabularData.swiftinterface:L509` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func map<T>(_ transform: (TabularData.Column<WrappedElement>.Element) throws -> T?) rethrows -> TabularData.Column<T>` | func | `TabularData.swiftinterface:L511` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func mapNonNil<T>(_ transform: (WrappedElement) throws -> T?) rethrows -> TabularData.Column<T>` | func | `TabularData.swiftinterface:L514` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func transform(_ transform: (TabularData.Column<WrappedElement>.Element) throws -> TabularData.Column<WrappedElement>.Element) rethrows` | func | `TabularData.swiftinterface:L517` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func transform(_ transform: (WrappedElement) throws -> WrappedElement) rethrows` | func | `TabularData.swiftinterface:L519` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func filter(_ isIncluded: (TabularData.Column<WrappedElement>.Element) throws -> Swift.Bool) rethrows -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | func | `TabularData.swiftinterface:L521` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L525` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.typealias Indices = Swift.Range<Swift.Int>` | typealias | `TabularData.swiftinterface:L527` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.typealias Iterator = Swift.IndexingIterator<TabularData.Column<WrappedElement>>` | typealias | `TabularData.swiftinterface:L529` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.typealias SubSequence = TabularData.ColumnSlice<WrappedElement>` | typealias | `TabularData.swiftinterface:L531` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L535` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L538` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L541` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L542` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.subscript(position: Swift.Int) -> TabularData.Column<WrappedElement>.Element` | subscript | `TabularData.swiftinterface:L544` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.subscript(bounds: Swift.Range<Swift.Int>) -> TabularData.ColumnSlice<WrappedElement>` | subscript | `TabularData.swiftinterface:L551` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.subscript<R>(range: R) -> TabularData.ColumnSlice<WrappedElement> where R : Swift.RangeExpression, R.Bound == Swift.Int` | subscript | `TabularData.swiftinterface:L555` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.subscript<C>(mask: C) -> TabularData.DiscontiguousColumnSlice<WrappedElement> where C : Swift.Collection, C.Element == Swift.Bool` | subscript | `TabularData.swiftinterface:L563` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func withContiguousStorageIfAvailable<R>(_ body: (Swift.UnsafeBufferPointer<WrappedElement?>) throws -> R) rethrows -> R?` | func | `TabularData.swiftinterface:L567` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func withContiguousMutableStorageIfAvailable<R>(_ body: (inout Swift.UnsafeMutableBufferPointer<WrappedElement?>) throws -> R) rethrows -> R?` | func | `TabularData.swiftinterface:L570` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func withContiguousStorageIfAvailable<R>(_ body: (Swift.UnsafeBufferPointer<WrappedElement>) throws -> R) rethrows -> R?` | func | `TabularData.swiftinterface:L574` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.mutating func withContiguousMutableStorageIfAvailable<R>(_ body: (inout Swift.UnsafeMutableBufferPointer<WrappedElement>) throws -> R) rethrows -> R?` | func | `TabularData.swiftinterface:L578` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func distinct() -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | func | `TabularData.swiftinterface:L587` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L588` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L589` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func encode(to encoder: any Swift.Encoder) throws` | func | `TabularData.swiftinterface:L595` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.init(from decoder: any Swift.Decoder) throws` | init | `TabularData.swiftinterface:L599` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnID` | struct | `TabularData.swiftinterface:L605` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `ColumnID.var name: Swift.String` | var | `TabularData.swiftinterface:L606` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `ColumnID.var type: any Any.Type` | var | `TabularData.swiftinterface:L608` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `ColumnID.init(_ name: Swift.String, _ type: T.Type)` | init | `TabularData.swiftinterface:L611` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `ColumnID.var description: Swift.String` | var | `TabularData.swiftinterface:L615` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `ColumnSlice` | struct | `TabularData.swiftinterface:L620` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.typealias Element = WrappedElement?` | typealias | `TabularData.swiftinterface:L621` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L622` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var name: Swift.String` | var | `TabularData.swiftinterface:L625` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var wrappedElementType: any Any.Type` | var | `TabularData.swiftinterface:L635` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var prototype: any TabularData.AnyColumnPrototype` | var | `TabularData.swiftinterface:L640` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.init(_ column: TabularData.Column<WrappedElement>)` | init | `TabularData.swiftinterface:L647` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func map<T>(_ transform: (TabularData.ColumnSlice<WrappedElement>.Element) throws -> T?) rethrows -> TabularData.Column<T>` | func | `TabularData.swiftinterface:L651` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func filter(_ isIncluded: (TabularData.ColumnSlice<WrappedElement>.Element) throws -> Swift.Bool) rethrows -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | func | `TabularData.swiftinterface:L656` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func eraseToAnyColumn() -> TabularData.AnyColumnSlice` | func | `TabularData.swiftinterface:L658` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.typealias Indices = Swift.Range<TabularData.ColumnSlice<WrappedElement>.Index>` | typealias | `TabularData.swiftinterface:L660` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.typealias Iterator = Swift.IndexingIterator<TabularData.ColumnSlice<WrappedElement>>` | typealias | `TabularData.swiftinterface:L662` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.typealias SubSequence = TabularData.ColumnSlice<WrappedElement>` | typealias | `TabularData.swiftinterface:L664` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L671` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L674` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L677` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L680` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var count: Swift.Int` | var | `TabularData.swiftinterface:L683` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var missingCount: Swift.Int` | var | `TabularData.swiftinterface:L689` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.subscript(position: Swift.Int) -> TabularData.ColumnSlice<WrappedElement>.Element` | subscript | `TabularData.swiftinterface:L695` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func isNil(at index: Swift.Int) -> Swift.Bool` | func | `TabularData.swiftinterface:L704` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.subscript(range: Swift.Range<Swift.Int>) -> TabularData.ColumnSlice<WrappedElement>` | subscript | `TabularData.swiftinterface:L705` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L720` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func distinct() -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | func | `TabularData.swiftinterface:L721` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L722` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DataFrame.Slice.func summary() -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1171` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func summary(of columnNames: Swift.String...) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1172` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.func summary(ofColumns columnIndices: Swift.Int...) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1173` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.mutating func combineColumns<E1, E2, R>(_ columnName1: Swift.String, _ columnName2: Swift.String, into newColumnName: Swift.String, transform: (E1?, E2?) throws -> R?) rethrows` | func | `TabularData.swiftinterface:L1178` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func combineColumns<E1, E2, R>(_ columnID1: TabularData.ColumnID<E1>, _ columnID2: TabularData.ColumnID<E2>, into newColumnName: Swift.String, transform: (E1?, E2?) throws -> R?) rethrows` | func | `TabularData.swiftinterface:L1181` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func combineColumns<E1, E2, E3, R>(_ columnName1: Swift.String, _ columnName2: Swift.String, _ columnName3: Swift.String, into newColumnName: Swift.String, transform: (E1?, E2?, E3?) throws -> R?) rethrows` | func | `TabularData.swiftinterface:L1184` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func combineColumns<E1, E2, E3, R>(_ columnID1: TabularData.ColumnID<E1>, _ columnID2: TabularData.ColumnID<E2>, _ columnID3: TabularData.ColumnID<E3>, into newColumnName: Swift.String, transform: (E1?, E2?, E3?) throws -> R?) rethrows` | func | `TabularData.swiftinterface:L1187` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `SFrameReadingError` | enum | `TabularData.swiftinterface:L1191` | SFrame import APIs are not wrapped. |
| `SFrameReadingError.var description: Swift.String` | var | `TabularData.swiftinterface:L1202` | SFrame import APIs are not wrapped. |
| `SFrameReadingError.var errorDescription: Swift.String?` | var | `TabularData.swiftinterface:L1209` | SFrame import APIs are not wrapped. |
| `DataFrame.init(csvData data: Foundation.Data, columns: [Swift.String]? = nil, rows: Swift.Range<Swift.Int>? = nil, types: [Swift.String : TabularData.CSVType] = [:], options: TabularData.CSVReadingOptions = .init()) throws` | init | `TabularData.swiftinterface:L1220` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.init<each T>(contentsOfCSVFile url: Foundation.URL, columns: repeat TabularData.ColumnID<each T>, rows: Swift.Range<Swift.Int>? = nil, options: TabularData.CSVReadingOptions = .init()) throws` | init | `TabularData.swiftinterface:L1224` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.init<each T>(csvData data: Foundation.Data, columns: repeat TabularData.ColumnID<each T>, rows: Swift.Range<Swift.Int>? = nil, options: TabularData.CSVReadingOptions = .init()) throws` | init | `TabularData.swiftinterface:L1228` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrameProtocol` | protocol | `TabularData.swiftinterface:L1232` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.var base: TabularData.DataFrame` | var | `TabularData.swiftinterface:L1234` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.var rows: TabularData.DataFrame.Rows` | var | `TabularData.swiftinterface:L1235` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.var columns: [Self.ColumnType]` | var | `TabularData.swiftinterface:L1236` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.var shape: (rows: Swift.Int, columns: Swift.Int)` | var | `TabularData.swiftinterface:L1237` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrame.var base: TabularData.DataFrame` | var | `TabularData.swiftinterface:L1242` | No public Rust wrapper for this SDK symbol. |
| `DataFrameProtocol.var isEmpty: Swift.Bool` | var | `TabularData.swiftinterface:L1248` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.subscript<R>(r: R) -> TabularData.DataFrame.Slice where R : Swift.RangeExpression, R.Bound == Swift.Int` | subscript | `TabularData.swiftinterface:L1255` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `AnyCategoricalSummary` | struct | `TabularData.swiftinterface:L1265` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.var someCount: Swift.Int` | var | `TabularData.swiftinterface:L1266` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.var noneCount: Swift.Int` | var | `TabularData.swiftinterface:L1267` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.var totalCount: Swift.Int` | var | `TabularData.swiftinterface:L1268` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.var uniqueCount: Swift.Int` | var | `TabularData.swiftinterface:L1271` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.var mode: [Any]` | var | `TabularData.swiftinterface:L1272` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.var modeType: any Any.Type` | var | `TabularData.swiftinterface:L1273` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.init<T>(_ summary: TabularData.CategoricalSummary<T>) where T : Swift.Hashable` | init | `TabularData.swiftinterface:L1274` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `AnyCategoricalSummary.init(_ summary: TabularData.CategoricalSummary<Swift.AnyHashable>)` | init | `TabularData.swiftinterface:L1275` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `Column.func decoded<T, Decoder>(_ type: T.Type, using decoder: Decoder) throws -> TabularData.Column<T> where WrappedElement == Decoder.Input, T : Swift.Decodable, Decoder : Combine.TopLevelDecoder` | func | `TabularData.swiftinterface:L1280` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.func decoded<T, Decoder>(_ type: T.Type, using decoder: Decoder) throws -> TabularData.AnyColumn where T : Swift.Decodable, Decoder : Combine.TopLevelDecoder` | func | `TabularData.swiftinterface:L1284` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.mutating func decode<T, Decoder>(_ type: T.Type, using decoder: Decoder) throws where T : Swift.Decodable, Decoder : Combine.TopLevelDecoder` | func | `TabularData.swiftinterface:L1285` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DataFrame.mutating func decode<T, Decoder>(_ type: T.Type, inColumn id: TabularData.ColumnID<Decoder.Input>, using decoder: Decoder) throws where T : Swift.Decodable, Decoder : Combine.TopLevelDecoder` | func | `TabularData.swiftinterface:L1290` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `ColumnDecodingError` | struct | `TabularData.swiftinterface:L1293` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnDecodingError.var columnName: Swift.String` | var | `TabularData.swiftinterface:L1294` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnDecodingError.var rowIndex: Swift.Int` | var | `TabularData.swiftinterface:L1295` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnDecodingError.var decodingError: Swift.DecodingError` | var | `TabularData.swiftinterface:L1296` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnDecodingError.init(columnName: Swift.String, rowIndex: Swift.Int, decodingError: Swift.DecodingError)` | init | `TabularData.swiftinterface:L1297` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnDecodingError.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L1298` | Dedicated SDK error enums are not surfaced as Rust types. |
| `GroupSummaries` | protocol | `TabularData.swiftinterface:L1303` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `GroupSummaries.subscript(keys: Any?...) -> TabularData.DataFrame?` | subscript | `TabularData.swiftinterface:L1305` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `GroupSummaries.var description: Swift.String` | var | `TabularData.swiftinterface:L1307` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `GroupSummaries.func description(options: TabularData.FormattingOptions) -> Swift.String` | func | `TabularData.swiftinterface:L1308` | Formatting and display helpers are not wrapped. |
| `DataFrameProtocol.func sorted<T>(on columnID: TabularData.ColumnID<T>, order: TabularData.Order = .ascending) -> TabularData.DataFrame where T : Swift.Comparable` | func | `TabularData.swiftinterface:L1314` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func sorted<T0, T1>(on columnID0: TabularData.ColumnID<T0>, _ columnID1: TabularData.ColumnID<T1>, order: TabularData.Order = .ascending) -> TabularData.DataFrame where T0 : Swift.Comparable, T1 : Swift.Comparable` | func | `TabularData.swiftinterface:L1315` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func sorted<T0, T1, T2>(on columnID0: TabularData.ColumnID<T0>, _ columnID1: TabularData.ColumnID<T1>, _ columnID2: TabularData.ColumnID<T2>, order: TabularData.Order = .ascending) -> TabularData.DataFrame where T0 : Swift.Comparable, T1 : Swift.Comparable, T2 : Swift.Comparable` | func | `TabularData.swiftinterface:L1316` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func sorted<T>(on columnName: Swift.String, _ type: T.Type, by areInIncreasingOrder: (T, T) throws -> Swift.Bool) rethrows -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1317` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func sorted<T>(on columnID: TabularData.ColumnID<T>, by areInIncreasingOrder: (T, T) throws -> Swift.Bool) rethrows -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1318` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrame.mutating func sort<T>(on columnID: TabularData.ColumnID<T>, order: TabularData.Order = .ascending) where T : Swift.Comparable` | func | `TabularData.swiftinterface:L1324` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.mutating func sort<T0, T1>(on columnID0: TabularData.ColumnID<T0>, _ columnID1: TabularData.ColumnID<T1>, order: TabularData.Order = .ascending) where T0 : Swift.Comparable, T1 : Swift.Comparable` | func | `TabularData.swiftinterface:L1325` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.mutating func sort<T0, T1, T2>(on columnID0: TabularData.ColumnID<T0>, _ columnID1: TabularData.ColumnID<T1>, _ columnID2: TabularData.ColumnID<T2>, order: TabularData.Order = .ascending) where T0 : Swift.Comparable, T1 : Swift.Comparable, T2 : Swift.Comparable` | func | `TabularData.swiftinterface:L1326` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.mutating func sort<T>(on columnID: TabularData.ColumnID<T>, by areInIncreasingOrder: (T, T) throws -> Swift.Bool) rethrows` | func | `TabularData.swiftinterface:L1327` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.mutating func sort<T>(on columnName: Swift.String, _ type: T.Type, by areInIncreasingOrder: (T, T) throws -> Swift.Bool) rethrows` | func | `TabularData.swiftinterface:L1328` | No public Rust wrapper for this SDK symbol. |
| `JSONReadingError` | enum | `TabularData.swiftinterface:L1331` | JSON IO and JSON-specific errors are not wrapped. |
| `JSONReadingError.var description: Swift.String` | var | `TabularData.swiftinterface:L1339` | JSON IO and JSON-specific errors are not wrapped. |
| `JSONReadingError.var errorDescription: Swift.String?` | var | `TabularData.swiftinterface:L1346` | JSON IO and JSON-specific errors are not wrapped. |
| `CategoricalSummary` | struct | `TabularData.swiftinterface:L1352` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.var someCount: Swift.Int` | var | `TabularData.swiftinterface:L1353` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.var noneCount: Swift.Int` | var | `TabularData.swiftinterface:L1354` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.var totalCount: Swift.Int` | var | `TabularData.swiftinterface:L1355` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.var uniqueCount: Swift.Int` | var | `TabularData.swiftinterface:L1358` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.var mode: [Element]` | var | `TabularData.swiftinterface:L1359` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.init()` | init | `TabularData.swiftinterface:L1360` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.init(someCount: Swift.Int, noneCount: Swift.Int, uniqueCount: Swift.Int, mode: [Element])` | init | `TabularData.swiftinterface:L1361` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L1362` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L1366` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `CategoricalSummary.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L1367` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `DataFrame.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L1379` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L1382` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.Slice.var description: Swift.String` | var | `TabularData.swiftinterface:L1388` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L1391` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Slice.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L1394` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.var description: Swift.String` | var | `TabularData.swiftinterface:L1400` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.func description(options: TabularData.FormattingOptions) -> Swift.String` | func | `TabularData.swiftinterface:L1403` | Formatting and display helpers are not wrapped. |
| `DataFrame.Row.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L1404` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L1407` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `CSVReadingError` | enum | `TabularData.swiftinterface:L1416` | Dedicated SDK error enums are not surfaced as Rust types. |
| `CSVReadingError.var row: Swift.Int` | var | `TabularData.swiftinterface:L1427` | Dedicated SDK error enums are not surfaced as Rust types. |
| `CSVReadingError.var column: Swift.Int?` | var | `TabularData.swiftinterface:L1431` | Dedicated SDK error enums are not surfaced as Rust types. |
| `CSVReadingError.var description: Swift.String` | var | `TabularData.swiftinterface:L1438` | Dedicated SDK error enums are not surfaced as Rust types. |
| `CSVReadingError.var errorDescription: Swift.String?` | var | `TabularData.swiftinterface:L1445` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ShapedData` | struct | `TabularData.swiftinterface:L1475` | ShapedData is not modeled in the Rust crate. |
| `ShapedData.init(shape: [Swift.Int], strides: [Swift.Int], contents: [Element])` | init | `TabularData.swiftinterface:L1479` | ShapedData is not modeled in the Rust crate. |
| `ShapedData.subscript(indices: Swift.Int...) -> Element` | subscript | `TabularData.swiftinterface:L1480` | ShapedData is not modeled in the Rust crate. |
| `ShapedData.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L1490` | ShapedData is not modeled in the Rust crate. |
| `ShapedData.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L1491` | ShapedData is not modeled in the Rust crate. |
| `DiscontiguousColumnSlice` | struct | `TabularData.swiftinterface:L1499` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.typealias Element = WrappedElement?` | typealias | `TabularData.swiftinterface:L1500` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L1501` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var name: Swift.String` | var | `TabularData.swiftinterface:L1502` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var wrappedElementType: any Any.Type` | var | `TabularData.swiftinterface:L1506` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var prototype: any TabularData.AnyColumnPrototype` | var | `TabularData.swiftinterface:L1511` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.init(_ column: TabularData.Column<WrappedElement>)` | init | `TabularData.swiftinterface:L1514` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.init(column: TabularData.Column<WrappedElement>, ranges: [Swift.Range<Swift.Int>])` | init | `TabularData.swiftinterface:L1515` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func map<T>(_ transform: (TabularData.DiscontiguousColumnSlice<WrappedElement>.Element) throws -> T?) rethrows -> TabularData.Column<T>` | func | `TabularData.swiftinterface:L1517` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func filter(_ isIncluded: (TabularData.DiscontiguousColumnSlice<WrappedElement>.Element) throws -> Swift.Bool) rethrows -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | func | `TabularData.swiftinterface:L1520` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func eraseToAnyColumn() -> TabularData.AnyColumnSlice` | func | `TabularData.swiftinterface:L1522` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L1529` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L1532` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L1535` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L1536` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var count: Swift.Int` | var | `TabularData.swiftinterface:L1537` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var missingCount: Swift.Int` | var | `TabularData.swiftinterface:L1541` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.subscript(position: Swift.Int) -> TabularData.DiscontiguousColumnSlice<WrappedElement>.Element` | subscript | `TabularData.swiftinterface:L1545` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func isNil(at index: Swift.Int) -> Swift.Bool` | func | `TabularData.swiftinterface:L1550` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.subscript(range: Swift.Range<Swift.Int>) -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | subscript | `TabularData.swiftinterface:L1551` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.subscript<R>(range: R) -> TabularData.DiscontiguousColumnSlice<WrappedElement> where R : Swift.RangeExpression, R.Bound == Swift.Int` | subscript | `TabularData.swiftinterface:L1555` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.subscript(range: (Swift.UnboundedRange_) -> ()) -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | subscript | `TabularData.swiftinterface:L1563` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.typealias Indices = Swift.DefaultIndices<TabularData.DiscontiguousColumnSlice<WrappedElement>>` | typealias | `TabularData.swiftinterface:L1572` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.typealias Iterator = Swift.IndexingIterator<TabularData.DiscontiguousColumnSlice<WrappedElement>>` | typealias | `TabularData.swiftinterface:L1574` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.typealias SubSequence = TabularData.DiscontiguousColumnSlice<WrappedElement>` | typealias | `TabularData.swiftinterface:L1576` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L1584` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func distinct() -> TabularData.DiscontiguousColumnSlice<WrappedElement>` | func | `TabularData.swiftinterface:L1585` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L1586` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func encoded<Encoder>(using encoder: Encoder) throws -> TabularData.Column<Encoder.Output> where Encoder : Combine.TopLevelEncoder` | func | `TabularData.swiftinterface:L1670` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.func encoded<T, Encoder>(_ type: T.Type, using encoder: Encoder) throws -> TabularData.AnyColumn where T : Swift.Encodable, Encoder : Combine.TopLevelEncoder` | func | `TabularData.swiftinterface:L1674` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.mutating func encode<T, Encoder>(_ type: T.Type, using encoder: Encoder) throws where T : Swift.Encodable, Encoder : Combine.TopLevelEncoder` | func | `TabularData.swiftinterface:L1675` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DataFrame.mutating func encodeColumn<T, Encoder>(_ id: TabularData.ColumnID<T>, using encoder: Encoder) throws where T : Swift.Encodable, Encoder : Combine.TopLevelEncoder` | func | `TabularData.swiftinterface:L1680` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `ColumnEncodingError` | struct | `TabularData.swiftinterface:L1683` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnEncodingError.var columnName: Swift.String` | var | `TabularData.swiftinterface:L1684` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnEncodingError.var rowIndex: Swift.Int` | var | `TabularData.swiftinterface:L1685` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnEncodingError.var encodingError: Swift.EncodingError` | var | `TabularData.swiftinterface:L1686` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnEncodingError.init(columnName: Swift.String, rowIndex: Swift.Int, encodingError: Swift.EncodingError)` | init | `TabularData.swiftinterface:L1687` | Dedicated SDK error enums are not surfaced as Rust types. |
| `ColumnEncodingError.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L1688` | Dedicated SDK error enums are not surfaced as Rust types. |
| `AnyColumnSlice` | struct | `TabularData.swiftinterface:L1693` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var name: Swift.String` | var | `TabularData.swiftinterface:L1694` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var wrappedElementType: any Any.Type` | var | `TabularData.swiftinterface:L1698` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var count: Swift.Int` | var | `TabularData.swiftinterface:L1701` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var missingCount: Swift.Int` | var | `TabularData.swiftinterface:L1705` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.func assumingType<T>(_ type: T.Type) -> TabularData.DiscontiguousColumnSlice<T>` | func | `TabularData.swiftinterface:L1708` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.func isNil(at index: Swift.Int) -> Swift.Bool` | func | `TabularData.swiftinterface:L1709` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L1710` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L1716` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L1719` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L1722` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L1723` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.subscript(position: Swift.Int) -> Any?` | subscript | `TabularData.swiftinterface:L1725` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.subscript(range: Swift.Range<Swift.Int>) -> TabularData.AnyColumnSlice` | subscript | `TabularData.swiftinterface:L1730` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L1735` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.typealias Element = Any?` | typealias | `TabularData.swiftinterface:L1737` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L1739` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.typealias Indices = Swift.Range<Swift.Int>` | typealias | `TabularData.swiftinterface:L1741` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.typealias Iterator = Swift.IndexingIterator<TabularData.AnyColumnSlice>` | typealias | `TabularData.swiftinterface:L1743` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.typealias SubSequence = TabularData.AnyColumnSlice` | typealias | `TabularData.swiftinterface:L1745` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var description: Swift.String` | var | `TabularData.swiftinterface:L1749` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L1752` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L1755` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumnSlice.func distinct() -> TabularData.AnyColumnSlice` | func | `TabularData.swiftinterface:L1761` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DataFrame.mutating func explodeColumn<T>(_ id: TabularData.ColumnID<T>) where T : Swift.Collection` | func | `TabularData.swiftinterface:L1765` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.mutating func explodeColumn<T>(_ name: Swift.String, _ type: T.Type) where T : Swift.Collection` | func | `TabularData.swiftinterface:L1766` | Additional DataFrame mutation/reshape APIs are not wrapped. |
| `DataFrame.func explodingColumn<T>(_ name: Swift.String, _ type: T.Type) -> TabularData.DataFrame where T : Swift.Collection` | func | `TabularData.swiftinterface:L1767` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.func explodingColumn<T>(_ id: TabularData.ColumnID<T>) -> TabularData.DataFrame where T : Swift.Collection` | func | `TabularData.swiftinterface:L1768` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `RowGroupingProtocol.var count: Swift.Int` | var | `TabularData.swiftinterface:L1772` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGroupingProtocol.func ungrouped() -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1773` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGroupingProtocol.func aggregated<Element, Result>(on columnNames: [Swift.String], naming: (Swift.String) -> Swift.String, transform: (TabularData.DiscontiguousColumnSlice<Element>) throws -> Result?) rethrows -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1778` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGroupingProtocol.func filter(_ isIncluded: (TabularData.DataFrame.Slice) throws -> Swift.Bool) rethrows -> Self` | func | `TabularData.swiftinterface:L1781` | Advanced grouping summaries/filtering/splits are not wrapped. |
| `RowGroupingProtocol.func mapGroups(_ transform: (TabularData.DataFrame.Slice) throws -> TabularData.DataFrame) rethrows -> Self` | func | `TabularData.swiftinterface:L1782` | Advanced grouping summaries/filtering/splits are not wrapped. |
| `RowGroupingProtocol.func randomSplit(by proportion: Swift.Double, seed: Swift.Int?) -> (Self, Self)` | func | `TabularData.swiftinterface:L1784` | Advanced grouping summaries/filtering/splits are not wrapped. |
| `RowGroupingProtocol.func summary() -> any TabularData.GroupSummaries` | func | `TabularData.swiftinterface:L1786` | Advanced grouping summaries/filtering/splits are not wrapped. |
| `RowGroupingProtocol.func summary(of columnNames: [Swift.String]) -> any TabularData.GroupSummaries` | func | `TabularData.swiftinterface:L1787` | Advanced grouping summaries/filtering/splits are not wrapped. |
| `RowGroupingProtocol.func sums<N>(_ columnID: TabularData.ColumnID<N>, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.AdditiveArithmetic, N : Swift.Comparable` | func | `TabularData.swiftinterface:L1799` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `RowGroupingProtocol.func means<N>(_ columnID: TabularData.ColumnID<N>, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.FloatingPoint` | func | `TabularData.swiftinterface:L1805` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `RowGroupingProtocol.func quantiles<N>(_ columnID: TabularData.ColumnID<N>, quantile: N, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.BinaryFloatingPoint` | func | `TabularData.swiftinterface:L1811` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `RowGroupingProtocol.func minimums<N>(_ columnID: TabularData.ColumnID<N>, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.Comparable` | func | `TabularData.swiftinterface:L1817` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `RowGroupingProtocol.func maximums<N>(_ columnID: TabularData.ColumnID<N>, order: TabularData.Order? = nil) -> TabularData.DataFrame where N : Swift.Comparable` | func | `TabularData.swiftinterface:L1823` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `RowGroupingProtocol.func aggregated<Element, Result>(on columnNames: Swift.String..., naming: (Swift.String) -> Swift.String, transform: (TabularData.DiscontiguousColumnSlice<Element>) throws -> Result?) rethrows -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1826` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGroupingProtocol.func aggregated<Element, Result>(on columnID: TabularData.ColumnID<Element>, into aggregatedColumnName: Swift.String? = nil, transform: (TabularData.DiscontiguousColumnSlice<Element>) throws -> Result) rethrows -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L1829` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `RowGroupingProtocol.func randomSplit(by proportion: Swift.Double) -> (Self, Self)` | func | `TabularData.swiftinterface:L1831` | Advanced grouping summaries/filtering/splits are not wrapped. |
| `Order.func areOrdered<T>(_ lhs: T, _ rhs: T) -> Swift.Bool where T : Swift.Comparable` | func | `TabularData.swiftinterface:L1837` | Hashable/equatable helper members are not surfaced separately in Rust. |
| `Order.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L1839` | Hashable/equatable helper members are not surfaced separately in Rust. |
| `Order.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L1840` | Hashable/equatable helper members are not surfaced separately in Rust. |
| `FilledColumn.func min() -> TabularData.FilledColumn<Base>.Element?` | func | `TabularData.swiftinterface:L1847` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func max() -> TabularData.FilledColumn<Base>.Element?` | func | `TabularData.swiftinterface:L1850` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func argmin() -> Base.Index?` | func | `TabularData.swiftinterface:L1853` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func argmax() -> TabularData.FilledColumn<Base>.Index?` | func | `TabularData.swiftinterface:L1856` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func sum() -> TabularData.FilledColumn<Base>.Element` | func | `TabularData.swiftinterface:L1861` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func mean() -> Swift.Double?` | func | `TabularData.swiftinterface:L1863` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> Swift.Double?` | func | `TabularData.swiftinterface:L1866` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func sum() -> TabularData.FilledColumn<Base>.Element` | func | `TabularData.swiftinterface:L1871` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func mean() -> TabularData.FilledColumn<Base>.Element?` | func | `TabularData.swiftinterface:L1873` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> TabularData.FilledColumn<Base>.Element?` | func | `TabularData.swiftinterface:L1876` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `OptionalColumnProtocol.func filled(with value: Self.WrappedElement) -> TabularData.FilledColumn<Self>` | func | `TabularData.swiftinterface:L1881` | The Swift protocol hierarchy is not modeled directly in Rust. |
| `FilledColumn` | struct | `TabularData.swiftinterface:L1884` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.typealias Element = Base.WrappedElement` | typealias | `TabularData.swiftinterface:L1885` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.typealias WrappedElement = Base.WrappedElement` | typealias | `TabularData.swiftinterface:L1886` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.var name: Swift.String` | var | `TabularData.swiftinterface:L1891` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.var startIndex: Base.Index` | var | `TabularData.swiftinterface:L1895` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.var endIndex: Base.Index` | var | `TabularData.swiftinterface:L1898` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func index(after i: Base.Index) -> Base.Index` | func | `TabularData.swiftinterface:L1901` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func index(before i: Base.Index) -> Base.Index` | func | `TabularData.swiftinterface:L1902` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.subscript(position: Base.Index) -> Base.WrappedElement` | subscript | `TabularData.swiftinterface:L1903` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.typealias Index = Base.Index` | typealias | `TabularData.swiftinterface:L1909` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.typealias Indices = Swift.DefaultIndices<TabularData.FilledColumn<Base>>` | typealias | `TabularData.swiftinterface:L1911` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.typealias Iterator = Swift.IndexingIterator<TabularData.FilledColumn<Base>>` | typealias | `TabularData.swiftinterface:L1913` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.typealias SubSequence = Swift.Slice<TabularData.FilledColumn<Base>>` | typealias | `TabularData.swiftinterface:L1915` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DataFrame.init(contentsOfSFrameDirectory url: Foundation.URL, columns: [Swift.String]? = nil, rows: Swift.Range<Swift.Int>? = nil) throws` | init | `TabularData.swiftinterface:L1923` | SFrame import APIs are not wrapped. |
| `CSVWritingOptions.init(includesHeader: Swift.Bool = true, nilEncoding: Swift.String = "", trueEncoding: Swift.String = "true", falseEncoding: Swift.String = "false", newline: Swift.String = "\n", delimiter: Swift.Character = ",")` | init | `TabularData.swiftinterface:L1980` | No public Rust wrapper for this SDK symbol. |
| `CSVWritingError` | enum | `TabularData.swiftinterface:L1990` | Dedicated SDK error enums are not surfaced as Rust types. |
| `CSVWritingError.var row: Swift.Int` | var | `TabularData.swiftinterface:L1992` | Dedicated SDK error enums are not surfaced as Rust types. |
| `CSVWritingError.var column: Swift.String?` | var | `TabularData.swiftinterface:L1996` | Dedicated SDK error enums are not surfaced as Rust types. |
| `CSVWritingError.var description: Swift.String` | var | `TabularData.swiftinterface:L2003` | Dedicated SDK error enums are not surfaced as Rust types. |
| `RowGrouping` | struct | `TabularData.swiftinterface:L2008` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.var description: Swift.String` | var | `TabularData.swiftinterface:L2009` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.init<D>(groups: [(GroupingKey?, D)], groupKeysColumnName: Swift.String) where D : TabularData.DataFrameProtocol` | init | `TabularData.swiftinterface:L2013` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func counts(order: TabularData.Order? = nil) -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L2016` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func aggregated<Element, Result>(on columnNames: [Swift.String], naming: (Swift.String) -> Swift.String, transform: (TabularData.DiscontiguousColumnSlice<Element>) throws -> Result?) rethrows -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L2019` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func ungrouped() -> TabularData.DataFrame` | func | `TabularData.swiftinterface:L2021` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func filter(_ isIncluded: (TabularData.DataFrame.Slice) throws -> Swift.Bool) rethrows -> TabularData.RowGrouping<GroupingKey>` | func | `TabularData.swiftinterface:L2023` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func mapGroups(_ transform: (TabularData.DataFrame.Slice) throws -> TabularData.DataFrame) rethrows -> TabularData.RowGrouping<GroupingKey>` | func | `TabularData.swiftinterface:L2024` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func randomSplit(by proportion: Swift.Double, seed: Swift.Int? = nil) -> (TabularData.RowGrouping<GroupingKey>, TabularData.RowGrouping<GroupingKey>)` | func | `TabularData.swiftinterface:L2034` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.init<D>(frame: D, columnName: Swift.String, timeUnit: Foundation.Calendar.Component) where GroupingKey == Swift.Int, D : TabularData.DataFrameProtocol` | init | `TabularData.swiftinterface:L2039` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L2043` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L2046` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2049` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2050` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.var count: Swift.Int` | var | `TabularData.swiftinterface:L2051` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.subscript(position: Swift.Int) -> (key: GroupingKey?, group: TabularData.DataFrame.Slice)` | subscript | `TabularData.swiftinterface:L2055` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.typealias Element = (key: GroupingKey?, group: TabularData.DataFrame.Slice)` | typealias | `TabularData.swiftinterface:L2060` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L2062` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.typealias Indices = Swift.Range<Swift.Int>` | typealias | `TabularData.swiftinterface:L2064` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.typealias Iterator = Swift.IndexingIterator<TabularData.RowGrouping<GroupingKey>>` | typealias | `TabularData.swiftinterface:L2066` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.typealias SubSequence = Swift.Slice<TabularData.RowGrouping<GroupingKey>>` | typealias | `TabularData.swiftinterface:L2068` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `NumericSummary` | struct | `TabularData.swiftinterface:L2071` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var someCount: Swift.Int` | var | `TabularData.swiftinterface:L2072` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var noneCount: Swift.Int` | var | `TabularData.swiftinterface:L2073` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var totalCount: Swift.Int` | var | `TabularData.swiftinterface:L2074` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var median: Element` | var | `TabularData.swiftinterface:L2077` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var firstQuartile: Element` | var | `TabularData.swiftinterface:L2078` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var thirdQuartile: Element` | var | `TabularData.swiftinterface:L2079` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var mean: Element` | var | `TabularData.swiftinterface:L2080` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var standardDeviation: Element` | var | `TabularData.swiftinterface:L2081` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var min: Element` | var | `TabularData.swiftinterface:L2082` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var max: Element` | var | `TabularData.swiftinterface:L2083` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.init()` | init | `TabularData.swiftinterface:L2084` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.init(someCount: Swift.Int, noneCount: Swift.Int, mean: Element, standardDeviation: Element, min: Element, max: Element, median: Element, firstQuartile: Element, thirdQuartile: Element)` | init | `TabularData.swiftinterface:L2085` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L2086` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L2090` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `NumericSummary.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L2091` | Summary value types are not bridged directly; Rust re-materializes its own summary structs. |
| `DataFrame.Row.var base: TabularData.DataFrame` | var | `TabularData.swiftinterface:L2101` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.subscript<T>(position: Swift.Int, type: T.Type) -> T?` | subscript | `TabularData.swiftinterface:L2106` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.subscript<T>(columnName: Swift.String, type: T.Type) -> T?` | subscript | `TabularData.swiftinterface:L2112` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.subscript(columnName: Swift.String) -> Any?` | subscript | `TabularData.swiftinterface:L2118` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.subscript<T>(columnID: TabularData.ColumnID<T>) -> T?` | subscript | `TabularData.swiftinterface:L2124` | ColumnID-based typed overloads are not wrapped; the crate uses string column names. |
| `DataFrame.Rows` | struct | `TabularData.swiftinterface:L2133` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L2134` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L2137` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.var count: Swift.Int` | var | `TabularData.swiftinterface:L2140` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2143` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2144` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.subscript(position: Swift.Int) -> TabularData.DataFrame.Row` | subscript | `TabularData.swiftinterface:L2145` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.subscript(bounds: Swift.Range<Swift.Int>) -> TabularData.DataFrame.Rows` | subscript | `TabularData.swiftinterface:L2149` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.typealias Element = TabularData.DataFrame.Row` | typealias | `TabularData.swiftinterface:L2154` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L2156` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.typealias Indices = Swift.DefaultIndices<TabularData.DataFrame.Rows>` | typealias | `TabularData.swiftinterface:L2158` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.typealias Iterator = Swift.IndexingIterator<TabularData.DataFrame.Rows>` | typealias | `TabularData.swiftinterface:L2160` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Rows.typealias SubSequence = TabularData.DataFrame.Rows` | typealias | `TabularData.swiftinterface:L2162` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L2167` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L2170` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2173` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2174` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.var count: Swift.Int` | var | `TabularData.swiftinterface:L2175` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.subscript(position: Swift.Int) -> Any?` | subscript | `TabularData.swiftinterface:L2179` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.subscript(bounds: Swift.Range<Swift.Int>) -> Swift.Slice<TabularData.DataFrame.Row>` | subscript | `TabularData.swiftinterface:L2184` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.typealias Element = Any?` | typealias | `TabularData.swiftinterface:L2189` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L2191` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.typealias Indices = Swift.Range<Swift.Int>` | typealias | `TabularData.swiftinterface:L2193` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.typealias Iterator = Swift.IndexingIterator<TabularData.DataFrame.Row>` | typealias | `TabularData.swiftinterface:L2195` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.typealias SubSequence = Swift.Slice<TabularData.DataFrame.Row>` | typealias | `TabularData.swiftinterface:L2197` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L2202` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrame.Row.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L2203` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `Column.func min() -> TabularData.Column<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2210` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func max() -> TabularData.Column<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2213` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func argmin() -> Swift.Int?` | func | `TabularData.swiftinterface:L2216` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func argmax() -> Swift.Int?` | func | `TabularData.swiftinterface:L2219` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func min() -> TabularData.DiscontiguousColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2225` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func max() -> TabularData.DiscontiguousColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2228` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func argmin() -> Swift.Int?` | func | `TabularData.swiftinterface:L2231` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func argmax() -> Swift.Int?` | func | `TabularData.swiftinterface:L2234` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func min() -> TabularData.ColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2240` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func max() -> TabularData.ColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2243` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func argmin() -> Swift.Int?` | func | `TabularData.swiftinterface:L2246` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func argmax() -> Swift.Int?` | func | `TabularData.swiftinterface:L2249` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func sum() -> WrappedElement` | func | `TabularData.swiftinterface:L2254` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func sum() -> WrappedElement` | func | `TabularData.swiftinterface:L2258` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func sum() -> WrappedElement` | func | `TabularData.swiftinterface:L2262` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func mean() -> TabularData.Column<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2267` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> TabularData.Column<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2270` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func mean() -> TabularData.DiscontiguousColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2276` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> TabularData.DiscontiguousColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2279` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func mean() -> TabularData.ColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2285` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> TabularData.ColumnSlice<WrappedElement>.Element` | func | `TabularData.swiftinterface:L2288` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func mean() -> Swift.Double?` | func | `TabularData.swiftinterface:L2294` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> Swift.Double?` | func | `TabularData.swiftinterface:L2297` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func mean() -> Swift.Double?` | func | `TabularData.swiftinterface:L2303` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> Swift.Double?` | func | `TabularData.swiftinterface:L2306` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func mean() -> Swift.Double?` | func | `TabularData.swiftinterface:L2312` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func standardDeviation(deltaDegreesOfFreedom: Swift.Int = 1) -> Swift.Double?` | func | `TabularData.swiftinterface:L2315` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `CSVType.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L2333` | No public Rust wrapper for this SDK symbol. |
| `CSVType.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L2334` | No public Rust wrapper for this SDK symbol. |
| `DataFrame.Slice.func grouped(by columnName: Swift.String) -> any TabularData.RowGroupingProtocol` | func | `TabularData.swiftinterface:L2344` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `DataFrameProtocol.func grouped<GroupingKey>(by columnID: TabularData.ColumnID<GroupingKey>) -> TabularData.RowGrouping<GroupingKey> where GroupingKey : Swift.Hashable` | func | `TabularData.swiftinterface:L2348` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func grouped<InputKey, GroupingKey>(by columnName: Swift.String, transform: (InputKey?) -> GroupingKey?) -> TabularData.RowGrouping<GroupingKey> where GroupingKey : Swift.Hashable` | func | `TabularData.swiftinterface:L2350` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func grouped<InputKey, GroupingKey>(by columnID: TabularData.ColumnID<InputKey>, transform: (InputKey?) -> GroupingKey?) -> TabularData.RowGrouping<GroupingKey> where GroupingKey : Swift.Hashable` | func | `TabularData.swiftinterface:L2353` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func grouped(by columnID: TabularData.ColumnID<Foundation.Date>, timeUnit: Foundation.Calendar.Component) -> TabularData.RowGrouping<Swift.Int>` | func | `TabularData.swiftinterface:L2356` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func grouped<T>(by columnIDs: TabularData.ColumnID<T>...) -> some TabularData.RowGroupingProtocol where T : Swift.Hashable` | func | `TabularData.swiftinterface:L2359` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func grouped<T0, T1>(by column0: TabularData.ColumnID<T0>, _ column1: TabularData.ColumnID<T1>) -> some TabularData.RowGroupingProtocol where T0 : Swift.Hashable, T1 : Swift.Hashable` | func | `TabularData.swiftinterface:L2361` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func grouped<T0, T1, T2>(by column0: TabularData.ColumnID<T0>, _ column1: TabularData.ColumnID<T1>, _ column2: TabularData.ColumnID<T2>) -> some TabularData.RowGroupingProtocol where T0 : Swift.Hashable, T1 : Swift.Hashable, T2 : Swift.Hashable` | func | `TabularData.swiftinterface:L2363` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `RowGroupingProtocol.func summary(of columnNames: Swift.String...) -> any TabularData.GroupSummaries` | func | `TabularData.swiftinterface:L2368` | Advanced grouping summaries/filtering/splits are not wrapped. |
| `RowGrouping.func summary() -> any TabularData.GroupSummaries` | func | `TabularData.swiftinterface:L2372` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `RowGrouping.func summary(of columnNames: [Swift.String]) -> any TabularData.GroupSummaries` | func | `TabularData.swiftinterface:L2373` | Rust uses AnyRow/DataFrame copies instead of direct nested collection bindings. |
| `Column.var description: Swift.String` | var | `TabularData.swiftinterface:L2418` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L2421` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L2424` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var description: Swift.String` | var | `TabularData.swiftinterface:L2430` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L2433` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L2436` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var description: Swift.String` | var | `TabularData.swiftinterface:L2442` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L2445` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L2448` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `OptionalColumnProtocol.func description(options: TabularData.FormattingOptions) -> Swift.String` | func | `TabularData.swiftinterface:L2454` | Formatting and display helpers are not wrapped. |
| `FilledColumn.var description: Swift.String` | var | `TabularData.swiftinterface:L2458` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L2461` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func description(options: TabularData.FormattingOptions) -> Swift.String` | func | `TabularData.swiftinterface:L2464` | Formatting and display helpers are not wrapped. |
| `JoinKind.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L2473` | Hashable/equatable helper members are not surfaced separately in Rust. |
| `JoinKind.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L2474` | Hashable/equatable helper members are not surfaced separately in Rust. |
| `DataFrameProtocol.func joined<R, T>(_ other: R, on columnID: TabularData.ColumnID<T>, kind: TabularData.JoinKind = .inner) -> TabularData.DataFrame where R : TabularData.DataFrameProtocol, T : Swift.Hashable` | func | `TabularData.swiftinterface:L2481` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `DataFrameProtocol.func joined<R, T>(_ other: R, on columnIDs: (left: TabularData.ColumnID<T>, right: TabularData.ColumnID<T>), kind: TabularData.JoinKind = .inner) -> TabularData.DataFrame where R : TabularData.DataFrameProtocol, T : Swift.Hashable` | func | `TabularData.swiftinterface:L2483` | The crate exposes concrete DataFrame APIs instead of the full protocol abstraction. |
| `AnyColumn.var prototype: any TabularData.AnyColumnPrototype` | var | `TabularData.swiftinterface:L2494` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.func assumingType<T>(_ type: T.Type) -> TabularData.Column<T>` | func | `TabularData.swiftinterface:L2504` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.func isNil(at index: Swift.Int) -> Swift.Bool` | func | `TabularData.swiftinterface:L2505` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.mutating func append(_ element: Any?)` | func | `TabularData.swiftinterface:L2507` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.mutating func append(contentsOf other: TabularData.AnyColumn)` | func | `TabularData.swiftinterface:L2509` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.mutating func append(contentsOf other: TabularData.AnyColumnSlice)` | func | `TabularData.swiftinterface:L2510` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.mutating func remove(at index: Swift.Int)` | func | `TabularData.swiftinterface:L2511` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.var hashValue: Swift.Int` | var | `TabularData.swiftinterface:L2512` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.var startIndex: Swift.Int` | var | `TabularData.swiftinterface:L2518` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.var endIndex: Swift.Int` | var | `TabularData.swiftinterface:L2521` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.func index(after i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2524` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.func index(before i: Swift.Int) -> Swift.Int` | func | `TabularData.swiftinterface:L2525` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.subscript(position: Swift.Int) -> Any?` | subscript | `TabularData.swiftinterface:L2527` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.func hash(into hasher: inout Swift.Hasher)` | func | `TabularData.swiftinterface:L2540` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.typealias Element = Any?` | typealias | `TabularData.swiftinterface:L2542` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.typealias Index = Swift.Int` | typealias | `TabularData.swiftinterface:L2544` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.typealias Indices = Swift.Range<Swift.Int>` | typealias | `TabularData.swiftinterface:L2546` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.typealias Iterator = Swift.IndexingIterator<TabularData.AnyColumn>` | typealias | `TabularData.swiftinterface:L2548` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.typealias SubSequence = TabularData.AnyColumnSlice` | typealias | `TabularData.swiftinterface:L2550` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.var description: Swift.String` | var | `TabularData.swiftinterface:L2554` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.var debugDescription: Swift.String` | var | `TabularData.swiftinterface:L2557` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `AnyColumn.var customMirror: Swift.Mirror` | var | `TabularData.swiftinterface:L2560` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func numericSummary() -> TabularData.NumericSummary<WrappedElement>` | func | `TabularData.swiftinterface:L2570` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func numericSummary() -> TabularData.NumericSummary<WrappedElement>` | func | `TabularData.swiftinterface:L2574` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func numericSummary() -> TabularData.NumericSummary<WrappedElement>` | func | `TabularData.swiftinterface:L2578` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func numericSummary() -> TabularData.NumericSummary<Base.WrappedElement>` | func | `TabularData.swiftinterface:L2582` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FilledColumn.func numericSummary() -> TabularData.NumericSummary<Swift.Double>` | func | `TabularData.swiftinterface:L2586` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `Column.func numericSummary() -> TabularData.NumericSummary<Swift.Double>` | func | `TabularData.swiftinterface:L2590` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `ColumnSlice.func numericSummary() -> TabularData.NumericSummary<Swift.Double>` | func | `TabularData.swiftinterface:L2594` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `DiscontiguousColumnSlice.func numericSummary() -> TabularData.NumericSummary<Swift.Double>` | func | `TabularData.swiftinterface:L2598` | Typed column collection APIs are only partially mirrored; this symbol has no direct Rust binding. |
| `FormattingOptions.var floatingPointFormatStyle: Foundation.FloatingPointFormatStyle<Swift.Double>` | var | `TabularData.swiftinterface:L2611` | Formatting and display helpers are not wrapped. |
| `FormattingOptions.var integerFormatStyle: Foundation.IntegerFormatStyle<Swift.Int>` | var | `TabularData.swiftinterface:L2616` | Formatting and display helpers are not wrapped. |
| `FormattingOptions.var dateFormatStyle: Foundation.Date.FormatStyle` | var | `TabularData.swiftinterface:L2621` | Formatting and display helpers are not wrapped. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `CSVWritingOptions.var dateFormat: Swift.String?` | var | `TabularData.swiftinterface:L1950` | Deprecated API intentionally skipped. | `@available(*, deprecated, message: "Use dateFormatter instead.")` |
| `CSVWritingOptions.init(includesHeader: Swift.Bool = true, dateFormat: Swift.String?, nilEncoding: Swift.String = "", trueEncoding: Swift.String = "true", falseEncoding: Swift.String = "false", newline: Swift.String = "\n", delimiter: Swift.Character = ",")` | init | `TabularData.swiftinterface:L1978` | Deprecated API intentionally skipped. | `@available(*, deprecated, message: "Use dateFormatter instead or dateFormat.")` |
