set export := true
NODE_OPTIONS := "--experimental-wasm-modules"
build-wasm:
    if [ -d wasm/pkg ]; then rm -r wasm/pkg; fi
    cd wasm; wasm-pack build
build-web:
    cd web; pnpm run build
build: build-wasm
    pnpm i # it won't update types without this
    just build-web

test: test-wasm
test-wasm:
    cd wasm; cargo test
dev:
    @# Node.js の SSR で必要。
    cd web; pnpm dev
preview:
    cd web; pnpm preview
