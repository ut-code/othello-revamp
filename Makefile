deploy: export NODE_OPTIONS="--experimental-wasm-modules"
deploy:
	cd wasm; wasm-pack build
	cd web; pnpm i --frozen-lockfile
	cd web; pnpm run build
