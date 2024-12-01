set export := true
NODE_OPTIONS := "--experimental-wasm-modules"
setup:
    pnpm i --frozen-lockfile
    just build-wasm; # required because web/package.json depends on this
    cd web; pnpm i --frozen-lockfile
clean: clean-root clean-web clean-wasm
clean-root:
    if [ -d node_modules ]; then rm -r node_modules; fi
clean-web:
    cd web; if [ -d node_modules ]; then rm -r node_modules; fi
    cd web; if [ -d .svelte-kit ]; then rm -r .svelte-kit; fi
    cd web; if [ -d dist ]; then rm -r dist; fi
clean-wasm:
    cd wasm; cargo clean
    cd wasm; if [ -d pkg ]; then rm -r pkg; fi
    cd wasm; if [ -d target ]; then rm -r target; fi

build-wasm:
    if [ -d wasm/pkg ]; then rm -r wasm/pkg; fi
    cd wasm; wasm-pack build
build-web:
    cd web; pnpm run build
build: build-wasm
    just clean-web # .vite cache issue (ig)
    cd web; pnpm i # it won't update types without this
    just build-web

test: test-wasm
test-wasm:
    cd wasm; cargo test
dev:
    @# Node.js の SSR で必要。
    cd web; pnpm dev
preview:
    cd web; pnpm preview
