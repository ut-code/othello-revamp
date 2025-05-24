## NOTEs

- wasm/pkg が gitignore されていません。
  理由: Cloudflare が wasm-pack を提供していないため、ローカルでビルドして Git で管理することで cloudflare で直接 wasm を読み取ることにしています。

- CI が落ちてるよ！
  知らん。直して。

## 開発の始め方

```sh
mise trust && mise install
just setup
just dev # start hot-reloading server (note that it doesn't hot reload rust code; just the frontend part)
just build # build everything
```
