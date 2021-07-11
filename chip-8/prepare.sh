#!/bin/bash
# Compile and move the wasm module in the JavaScript source folder.
make -B;
mv interpreter.wasm ../view/src;