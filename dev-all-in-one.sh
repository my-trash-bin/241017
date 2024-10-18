#!/bin/sh

set -e

wasm-pack build wasm-worker
wasm-pack build wasm-all-in-one
(cd web-all-in-one && npm i && npx vite build && npx http-server dist)
