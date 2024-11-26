build-wasm:
    cd wasm; wasm-pack build
build-web:
    cd web; pnpm run build
build: build-wasm
    just copy-wasm
    just build-web
copy-wasm:
    if [ -d web/src/wasm ]; then rm -r web/src/wasm; fi
    cp wasm/pkg web/src/wasm -r
