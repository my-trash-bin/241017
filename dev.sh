#!/bin/sh

set -e

wasm-pack build wasm-worker
(cd web-worker && npm i && npx vite build)
wasm-pack build wasm-main
(cd web-main && npm i && npx vite dev)
