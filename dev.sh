#!/bin/sh

set -e

wasm-pack build worker
(cd web-worker && npm i && npx vite build)
wasm-pack build main
(cd web-main && npm i && npx vite dev)
