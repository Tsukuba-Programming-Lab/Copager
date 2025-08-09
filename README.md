# Copager

Rust製組み込み型パーサジェネレータ（**Constructible** **Pa**rser **Ge**nerator on **R**ust）

## Examples

- [example_build_oneshot](examples/build_oneshot) [(main.rs)](examples/build_oneshot/src/main.rs)
- [example_build_prebuild](examples/build_prebuild) [(main.rs)](examples/build_prebuild/src/main.rs)
- [example_lang_easyarith](examples/lang_easyarith) [(syntax.rs)](examples/lang_easyarith/src/syntax.rs)
- [example_lang_json](examples/lang_json) [(syntax.rs)](examples/lang_json/src/syntax.rs)
- [example_lang_pl0](examples/lang_pl0) [(syntax.rs)](examples/lang_pl0/src/syntax.rs)
- [example_lang_xml](examples/lang_xml) [(syntax.rs)](examples/lang_xml/src/syntax.rs)

```
$ cargo run -p example_lang_easyarith
var x;
var y;
x = 10;
y = 0x10;
print (x + x) * (y + y);
640
```

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
- `tree` : [crates/ir_tree](crates/ir_tree)

```
// RegexLex(lex) + LR1(parse) + SExp(ir)
copager = { ..., features = ["derive", "regexlex", "lr1", "sexp"] }

// RegexLex(lex) + LALR1(parse) + Void(ir)
copager = { ..., features = ["derive", "regexlex", "lalr1", "void"] }
```

## Test

```
$ cargo test
```
