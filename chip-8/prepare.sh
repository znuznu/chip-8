#!/bin/bash
# Compile and move the wasm module in the JavaScript source folder.
make -B;
mv target/wasm32-unknown-unknown/release/chip_8.wasm ../view/src;