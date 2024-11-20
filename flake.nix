# Nix Flake: 依存関係を統一的に管理し、バ＝ジョンを固定する。
# Nix 公式はドキュメントなんて丁寧なものを書いてくれないので、適当な Zenn の記事読み漁るか Youtube で Nix 人の動画見るかして勉強してください。

# Init Command: `nix flake init`
# 必須: flake.lock
# 推奨: .envrc
{
  description = "Templates"; # パッケージの説明。置き換えなくても動くが、多分置き換えたいだろう。

  inputs = {
    # <template repeat desc="outputs の引数に入る入力のリスト" example="fenix.url=\"...\"" />
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
      toolchain = import ./toolchain.nix { inherit system fenix; };
    in
    {
      # パッケージの設定
      # <template optional />
      # `nix build` を実行すると /nix/store に pkgs.hello の artifact が生成され、`nix run` を実行すると stdout に `Hello world!` が出力されるはずです。
      packages.default = pkgs.hello;
      # More examples: 
      # - simple stdenv.mkDerivation: https://qiita.com/hnakano863/items/4abfa4475ddc4c8684b1
      # - rustPlatform.buildRustPackage: https://zenn.dev/asa1984/books/nix-hands-on/viewer/ch04-02-rust-project
      # - buildGoModule: https://nixos.org/manual/nixpkgs/stable/#sec-language-go
      # - node2nix: https://zenn.dev/pandaman64/articles/zenn-built-with-nix

      # 開発シェルの設定
      devShell = pkgs.mkShell {
        name = "Template Development Shell"; # devShell の名前
        buildInputs = with pkgs; [
          # <template repeat desc="開発中に使うパッケージ" example="biome" />
          bun
          toolchain
        ];
      };
    });
}
