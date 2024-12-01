deploy: export NODE_OPTIONS="--experimental-wasm-modules"

deploy:
	@# https://www.npmjs.com/package/wasm-pack lmao
	pnpm wasm-pack build ${PWD}/wasm
	cd web; pnpm i --frozen-lockfile
	cd web; pnpm run build
