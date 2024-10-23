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

### build/one-shot

[examples/build_oneshot](examples/build_oneshot)

```
$ echo "10 * (20 + 30)" | cargo run -p example_build_oneshot
Success : (Expr (Term (Term (Num "10")) "*" (Num "(" (Expr (Expr (Term (Num "20"))) "+" (Term (Num "30"))) ")")))
```

### build/pre-build

[examples/build_prebuild](examples/build_prebuild)

```
$ echo "10 * (20 + 30)" | cargo run -p example_build_prebuild
Success : (Expr (Term (Term (Num "10")) "*" (Num "(" (Expr (Expr (Term (Num "20"))) "+" (Term (Num "30"))) ")")))
```

## Docs

```
$ make -C docs run
```

⇒ http://localhost:1313
