# Copager

Rust製パーサジェネレータ

## Features

### Common

- `derive`
- `all`

### Lex

- `regexlex` : [crates/lex_regex](crates/lex_regex)

### Parse

- `lr1` : [crates/parse_lr1](crates/parse_lr1)

### IR

- `void` : [crates/ir_void](crates/ir_void)
- `sexp` : [crates/ir_sexp](crates/ir_sexp)

## Examples

[examples/oneshot](examples/oneshot)

### ok

```
$ echo "(10 * (20 + 30)) / (40 + 60)" | cargo run -p example_oneshot
(10 * (20 + 30)) / (40 + 60) = 5
```

### error

```
$ echo "(10 -)" | cargo run -p example_oneshot
Error: Unexpected token "BracketR" found
-----
 1: (10 - )
          ^ here
Found at line 1, column 7.
-----
```
