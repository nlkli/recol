#!/bin/bash

git clone --depth 1 https://github.com/mbadolato/iTerm2-Color-Schemes && \
    mv iTerm2-Color-Schemes/ghostty ./colorschemes && \
    rm -rf iTerm2-Color-Schemes
