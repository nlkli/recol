#!/bin/bash
set -e

# Fetch the latest Ghostty themes.
git clone --depth 1 https://github.com/mbadolato/iTerm2-Color-Schemes
mv iTerm2-Color-Schemes/ghostty ./colorschemes
rm -rf iTerm2-Color-Schemes

# Add custom themes to ./colorschemes if needed.
# Unwanted themes can be filtered in build.rs, e.g.:
# |name| !["theme_to_exclude"].contains(&name)

# Build the color schemes binary:
# RECOL_BUILD_COLORSCHEMES_BIN=1 cargo build --release

