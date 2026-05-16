# Changelog

## [0.1.0] - 2026-05-16

- Initial release of `tabulardata-rs`.
- Safe Rust wrappers for `DataFrame`, column construction, CSV reading/writing options, joins, and summary generation.
- SwiftPM bridge for the pure-Swift `TabularData.framework` surface.
- Smoke example that writes `target/tabular.csv`, reloads it, and joins two frames without using `/tmp`.
