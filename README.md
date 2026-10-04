# veil-env

[English](README.en.md)

外部ボルトの値を子プロセスの環境変数へ渡し、手作業の `.env` 保存を減らすためのRust CLIです。
現在は **Mockのみの初期実装** です。AWS・1Password・veil-dropとの接続は未実装です。

## 準備

Nix利用者は `nix develop path:.`、非Nix利用者はrustupを導入して以下を実行します。
Rustは `rust-toolchain.toml`、Cargo依存は `Cargo.lock`、Nix入力は `flake.lock` で固定します。

```sh
cargo build --locked
cargo run --locked -- run --source mock:demo -- npm run dev
```

子プロセスには `VEIL_ENV_DEMO_SECRET=dummy-not-a-real-secret` が注入されます。
`npm run dev` は既存のアプリで実行する例であり、このリポジトリにNodeアプリは含まれません。

```sh
# macOS / Linux: 親の環境をクリアして起動
cargo run --locked -- run --source mock:demo --clear-env -- /bin/sh -c 'test -n "$VEIL_ENV_DEMO_SECRET"'
```

既定では親の環境を継承し、同名キーを取得値で上書きします。コマンドは `--` の後に指定します。
シェルを自動で挟まないため、パイプや展開が必要ならシェルを明示してください。
標準入出力を直接継承し、通常終了コードを引き継ぎます。

## 安全性と制約

ラッパーはシークレットをファイルへ保存しません。ただし値は親と子のメモリに存在し、
環境変数の閲覧、swap、ダンプ、子による保存・送信を防ぐものではありません。
親をKILLしても子が終了する保証はなく、明示的なシグナル転送も未実装です。
詳細は [セキュリティモデル](docs/SECURITY_MODEL.md) を参照してください。
実シークレットを扱う前に、この制約とプロバイダー設計をレビューする必要があります。

## 開発

```sh
just lint
just test
just build
just lint-md
```

[開発規約](CONTRIBUTING.md)、[運用ガイド](docs/GUIDELINES.md)、
[想定ユーザー](docs/TARGET_AUDIENCE.md)、[脆弱性報告](SECURITY.md) を参照してください。
CIはmacOS・Linux・Windows向けの検証を定義しています。実行結果は別途確認が必要です。
cargo-distによる配布は次段階で準備し、公開承認後に有効化します。

MIT License。
