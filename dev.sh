#!/bin/sh

set -e

wasm-pack build worker
(cd web && npm i && npx vite dev)
