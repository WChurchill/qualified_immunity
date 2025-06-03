#!/bin/bash

cargo build -r --target wasm32-unknown-unknown 

wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/qualified_immunity.wasm 

rm -rf ./webbuild/assets 
cp -r assets ./webbuild 

zip -r webbuild.zip webbuild
