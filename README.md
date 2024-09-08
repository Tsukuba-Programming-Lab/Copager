# Copager

Rust製パーサジェネレータ

## Features

### Common

- `all`
- `derive`
- `prebuild`

### Lex

- `regexlex` : [crates/lex_regex](crates/lex_regex)

### Parse

- `lr1` : [crates/parse_lr1](crates/parse_lr1)

### IR

- `void` : [crates/ir_void](crates/ir_void)
- `sexp` : [crates/ir_sexp](crates/ir_sexp)

## Examples

### One-shot

[examples/oneshot](examples/oneshot)

```
$ echo "10 * (20 + 30)" | cargo run -p example_oneshot
Success : (Expr (Term (Term (Num "10")) "*" (Num "(" (Expr (Expr (Term (Num "20"))) "+" (Term (Num "30"))) ")")))
```

### Pre-build

[examples/prebuild](examples/prebuild)

```
$ echo "10 * (20 + 30)" | cargo run -p example_prebuild
Success : (Expr (Term (Term (Num "10")) "*" (Num "(" (Expr (Expr (Term (Num "20"))) "+" (Term (Num "30"))) ")")))
```
