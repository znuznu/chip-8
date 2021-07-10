#!/bin/bash
rustc --target wasm32-unknown-unknown -O --crate-type=cdylib src/lib.rs -o ../client/src/interpreter.wasm