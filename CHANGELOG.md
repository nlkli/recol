# Changelog

## v0.2.4 — 2026-09-10

### Commit `b5140c9`

- Renamed `store` module to `state`; moved history persistence into a new `src/state.rs`.
- Lazily-initialized state dir via `static mut` buffers; returns `&'static` slices.
- Added `RECOL_STATE_DIR` env var to override the state directory.
- Removed `utils::io_other_error`.
- Theme application is now transactional: backups are created before writing and restored on failure.
- Renamed target methods: `apply_theme` → `apply_theme_to`, `set_font` → `set_font_on`, `config_path` → `existing_default_config_path`.
- New helpers: `with_specific_or_for_all` and `with_existing_default_config_path`.
- Added `Error::InvalidPpm`. `ppm.rs` now uses `crate::Result`.
- Moved `print_palette`; dropped panicking `flush`.
- Added `parse_theme_adjustment` tests.
- Cosmetic: comment alignment and doc updates.
