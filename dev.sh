#!/bin/sh

set -e

wasm-pack build wasm-worker
wasm-pack build wasm-main
(cd web-main && npm i && npx vite build && npx http-server dist)
