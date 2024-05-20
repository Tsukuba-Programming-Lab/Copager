# Parsergen

Rust製パーサジェネレータ

## Features

- `derive`

## Examples

[examples/expr.rs](examples/expr.rs)

```
$ cargo run --example expr
(10+20)/((30*40)-50)
Accepted : (Expr (Term (Term (Num "(" (Expr (Expr (Term (Num "10"))) "+" (Term (Num "20"))) ")")) "/" (Num "(" (Expr (Expr (Term (Num "(" (Expr (Term (Term (Num "30")) "*" (Num "40"))) ")"))) "-" (Term (Num "50"))) ")")))
$ cargo run --example expr
10**
-----
 1: 10**
       ^ here
Error at line 1, column 4.
-----

Rejected : Unexpected token "Mul" found
```
