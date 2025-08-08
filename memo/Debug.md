# RustのDebugについて

## 1. `#[derive(Debug)]`
- 構造体や列挙型に実装を自動生成する属性マクロ
- `println!("{:?}", val)` でデバッグ出力できるようになる

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

## 2. `println!`のフォーマット指定子

| 書式 | 説明 | 例 |
|------|------|-----|
| `{:?}` | Debugトレイトによる簡易表示 | `println!("{:?}", val);` |
| `{:#?}` | Debugトレイトによる整形（複数行） | `println!("{:#?}", val);` |
| `{}` | Displayトレイトによる表示（要実装） | `println!("{}", val);` |

## 3. `std::fmt::Debug`トレイト
- カスタム型に対して手動で実装も可能
- フォーマット用の`fmt`関数を実装する

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}
```

## 4. `dbg!`マクロ
- 引数の値を標準エラー出力にデバッグ表示して、その値を返す
- 主に開発中の一時的なデバッグ用

```rust
let x = 5;
let y = dbg!(x * 2) + 1;
// 出力: [src/main.rs:10] x * 2 = 10
```

## 5. `#[derive(Debug)]`で注意点
- 構造体やenum内のフィールドも`Debug`を実装している必要がある
- もしフィールドだけ`Debug`未実装の場合はエラーになる

## 6. `pretty print` と `compact print`

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = Person {
        name: "Alice".to_string(),
        age: 30
    };
    
    println!("{:?}", p);   // コンパクト表示
    println!("{:#?}", p);  // 複数行の整形表示（pretty print）
}
```
