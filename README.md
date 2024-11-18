# Copager

「言語処理系生成系」の生成系（**Constructible** **Pa**rser **Ge**nerator on **R**ust）

## Features

### Common

- `all`
- `derive`
- `prebuild`
- `template`
- `dev`

### Lex

- `regexlex` : [crates/lex_regex](crates/lex_regex)

### Parse

- `lr0` : [crates/parse_lr_lr0](crates/parse_lr_lr0)
- `lr1` : [crates/parse_lr_lr1](crates/parse_lr_lr1)
- `slr1` : [crates/parse_lr_slr1](crates/parse_lr_slr1)
- `lalr1` : [crates/parse_lr_lalr1](crates/parse_lr_lalr1)

### IR

- `void` : [crates/ir_void](crates/ir_void)
- `sexp` : [crates/ir_sexp](crates/ir_sexp)

```
// RegexLex(lex) + LR1(parse) + SExp(ir)
copager = { ..., features = ["derive", "regexlex", "lr1", "sexp"] }

// RegexLex(lex) + LALR1(parse) + Void(ir)
copager = { ..., features = ["derive", "regexlex", "lalr1", "void"] }
```

## Examples

- [example_build_oneshot](examples/build_oneshot)
- [example_build_prebuild](examples/build_prebuild)
- [example_lang_arithmetic](examples/lang_arithmetic) [(lib.rs)](examples/lang_arithmetic/src/lib.rs)
- [example_lang_json](examples/lang_json) [(lib.rs)](examples/lang_json/src/lib.rs)
- [example_lang_pl0](examples/lang_pl0) [(lib.rs)](examples/lang_pl0/src/lib.rs)
- [example_lang_xml](examples/lang_xml) [(lib.rs)](examples/lang_xml/src/lib.rs)

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
