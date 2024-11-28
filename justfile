set export := true
NODE_OPTIONS := "--experimental-wasm-modules"
build-wasm:
    if [ -d wasm/pkg ]; then rm -r wasm/pkg; fi
    cd wasm; wasm-pack build
build-web:
    cd web; pnpm run build
build: build-wasm
    just copy-wasm
    just build-web
    pnpm i # it won't update types without this
copy-wasm:
    if [ -d web/src/wasm ]; then rm -r web/src/wasm; fi
    cp wasm/pkg web/src/wasm -r

test: test-wasm
test-wasm:
    cd wasm; cargo test
dev:
    @# Node.js の SSR で必要。
    cd web; pnpm dev
preview:
    NODE_OPTIONS=--experimental-wasm-modules cd web; pnpm preview
