# Parsergen

Rust製パーサジェネレータ

## Features

- `derive`

## Examples

[examples/expr.rs](examples/expr.rs)

```
$ cargo run --example expr
(10+20)/((30*40)-50)
Accepted

$ cargo run --example expr
10**
Rejected: Error at (0, 3)
```
