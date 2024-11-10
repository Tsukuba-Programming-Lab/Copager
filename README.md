# Copager

Rust製パーサジェネレータ

## Features

### Common

- `all`
- `derive`
- `prebuild`
- `dev`

### Lex

- `regexlex` : [crates/lex_regex](crates/lex_regex)

### Parse

- `lr0` : [crates/parse_lr0](crates/parse_lr0)
- `lr1` : [crates/parse_lr1](crates/parse_lr1)

### IR

- `void` : [crates/ir_void](crates/ir_void)
- `sexp` : [crates/ir_sexp](crates/ir_sexp)

## Examples

- [example_build_oneshot](examples/build_oneshot)
- [example_build_prebuild](examples/build_prebuild)
- [example_lang_arithmetic](examples/lang_arithmetic)
- [example_lang_json](examples/lang_json)
- [example_lang_pl0](examples/lang_pl0)
- [example_lang_xml](examples/lang_xml)

```
$ cargo run -p example_build_oneshot
Example <one-shot>
Input: 10 * 20 + 30
Success: (Expr (Expr (Term (Term (Num "10")) "*" (Num "20"))) "+" (Term (Num "30")))
```

## Test

```
$ cargo test
```
