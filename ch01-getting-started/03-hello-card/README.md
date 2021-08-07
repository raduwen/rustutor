## Cargo
- ビルドシステム
- パッケージマネージャ

```sh
$ cargo --version
cargo 1.53.0
```

## プロジェクトの作成
```sh
$ cargo new {PROJECT_NAME}
$ tree -I target
.
├── Cargo.toml
├── README.md
└── src
    └── main.rs 
```

### Cargo.toml
```toml
[package]
name = "hello_card"
version = "0.1.0"
edition = "2018"

[dependencies]
```

#### [package]
プログラムをコンパイルするために必要な構成情報。
- `name` : パッケージ名
- `version` : バージョン
- `edition` : Rustのエディション
- `author` : 環境から名前とメアドを取得するので正しくない場合は自分で修正する必要あり

#### [dependencies]
パッケージ(crate)の依存関係。

## ビルドと実行
### ビルド
```sh
$ cargo build
```
`target/debug/{package.name}`に作成される。

#### リリースビルド
```sh
$ cargo build --release
```
`target/release/{package.name}`に作成される。
最適化される。

### 実行
```sh
$ cargo run
```
ビルドして実行する。ビルド済みであればビルドは実行されない。

### チェック
```sh
$ cargo check
```
実行ファイルを生成せずに、コードのチェックを行なう。
