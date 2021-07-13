#!/bin/bash
npm run build;
cp -r games dist/;
cp src/chip_8.wasm dist/src;
