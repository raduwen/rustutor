## use
標準ライブラリの`std::io`をスコープに入れる。
```rust
use std::io
```

今回は、`std::io::read_line()`を使いたかったが、長いから`use`でスコープ内に入れて、`io::read_line()`と書けるようにしたイメージ。

## 変数
### 作成
変数の作成(`let`)
```rust
let a = b
```

### 束縛
```rust
let a = "b";
let b = "a";
```

### 可変性
`let`はデフォルトで不変。`mut`を付けることで値を変更できる。
```rust
let a = "a";     // immutable
let mut b = "b"; // mutable
```

### memo
```rust
let a = "a";
let a = "b";
```
これは値を変更しているわけではなく、再度束縛している。(シャドウイング)

## 型の関数
```rust
String::new();
```
ここで`new`は`String`型の関数。静的メソッド。

## Result
`std::io::read_line()`は`Result`型を返す。(正確には`std::io::Result<usize>`)
`Result`は列挙型で、バリアントは`Ok`と`Err`。
`Ok`は操作が成功したことを示し、結果の値を持つ。
`Err`は操作が失敗したことを示し、失敗理由に関する情報を持つ。
```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
`Result.expect()`は`Ok`の場合結果の値を返し、`Err`の場合プログラムをエラーメッセージを出力しクラッシュさせる。


## println!のplaceholder
`{}`はplaceholderで引数の値を埋め込むことができる。複数のplaceholderを使用した場合、引数順に割り当てられる。
```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

----

## cargo
Cargo.tomlの`[dependencies]`に依存crateを追加することで簡単に依存関係を解決できる。

```toml
[dependencies]
rand = "0.8.3"
```
のように`crate = "version"`と書く。
versionは[semver](https://semver.org)である。
versionは`0.8.3`と記述した場合、`^0.8.3`と同義で0.8.3以上で、0.9.0未満であることを表す。

### build
`cargo build`でcrateもダウンロードされビルドされる。
ビルド済みのものは中間ファイルが生成されるので二度ビルドされない。

### Cargo.lock
crateは更に他のcrateに依存している場合が殆どだ。
自身のpackageの依存関係に加えてdependenciesのcrateも含めたcrateの依存関係とversion固定することができる。

### cargo update
`cargo update` することで最新のcrateに更新することができる。
Cargo.lockも更新される。

## match
```rust
match value {
    arm1 => expression,
    arm2 => expression,
    ...
}
```
switchの強力版みたいなもの。
値を持つenumの値を取り出すことも可能。例えばResultでは以下のように書ける。
```rust
loop {
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
        },
    };
}
```
