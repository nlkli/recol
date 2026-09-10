# Changelog

## v0.2.4 — 2026-09-10

### State management
- Renamed `store` module to `state`; moved history persistence into a new `src/state.rs`.
- Lazily-initialized state dir via `static mut` buffers; returns `&'static` slices.
- Added `RECOL_STATE_DIR` env var to override the state directory.
- Removed `utils::io_other_error`.

### Theme/targets
- Theme application is now transactional: backups are created before writing and restored on failure.
- Renamed target methods: `apply_theme` → `apply_theme_to`, `set_font` → `set_font_on`, `config_path` → `existing_default_config_path`.
- New helpers: `with_specific_or_for_all` and `with_existing_default_config_path`.

### Ghostty
- Fix parsing of palette lines (`palette = N=#RRGGBB`); malformed lines are preserved instead of dropped.

### PPM / errors
- Added `Error::InvalidPpm`. `ppm.rs` now uses `crate::Result`.

### Color
- Moved `print_palette`; dropped panicking `flush`.

### CLI / args
- Added `--media <path>` and `--palettegen <n>` arguments.

### Other
- Added `parse_theme_adjustment` tests.
- Cosmetic: comment alignment and doc updates.
