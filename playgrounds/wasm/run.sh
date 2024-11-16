#!/usr/bin/env bash
wasm-tools parse -o $1.wasm $1.wat && node run.js $1
