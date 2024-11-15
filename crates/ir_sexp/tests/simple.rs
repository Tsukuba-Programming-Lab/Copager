use copager_core::{Language, Processor};
use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleTag, Rule, RuleElem};
use copager_lex::LexSource;
use copager_lex_regex::RegexLexer;
use copager_parse::ParseSource;
use copager_parse_lr_lr1::LR1;
use copager_ir_sexp::SExp;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, LexSource)]
enum ExprToken {
    #[default]
    #[token(text = r"\+")]
    Plus,
    #[token(text = r"-")]
    Minus,
    #[token(text = r"\*")]
    Mul,
    #[token(text = r"/")]
    Div,
    #[token(text = r"\(")]
    BracketL,
    #[token(text = r"\)")]
    BracketR,
    #[token(text = r"[1-9][0-9]*")]
    Num,
    #[token(text = r"[ \t\n]+", ignored)]
    _Whitespace,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, ParseSource)]
enum ExprRule {
    #[default]
    #[rule("<expr> ::= <expr> Plus <term>")]
    #[rule("<expr> ::= <expr> Minus <term>")]
    #[rule("<expr> ::= <term>")]
    Expr,
    #[rule("<term> ::= <term> Mul <num>")]
    #[rule("<term> ::= <term> Div <num>")]
    #[rule("<term> ::= <num>")]
    Term,
    #[rule("<num> ::= BracketL <expr> BracketR")]
    #[rule("<num> ::= Num")]
    Num,
}

#[test]
fn simple_display() {
    let ir = parse("1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"(Expr (Term (Num "1")))"#);

    let ir = parse("1 + 1");
    assert!(ir.is_ok());
    assert_eq!(ir.unwrap().to_string(), r#"(Expr (Expr (Term (Num "1"))) "+" (Term (Num "1")))"#);
}

#[test]
fn simple_eval() {
    assert_eq!(eval(&parse("1").unwrap()), 1);
    assert_eq!(eval(&parse("1 + 2").unwrap()), 3);
    assert_eq!(eval(&parse("1 + 2 * 3").unwrap()), 7);
    assert_eq!(eval(&parse("(1 + 2) * 3").unwrap()), 9);
}

fn parse<'input>(input: &'input str) -> anyhow::Result<SExp<'input, ExprToken, ExprRule>> {
    type TestLang = Language<ExprToken, ExprRule>;
    type TestLexer = RegexLexer<ExprToken>;
    type TestParser = LR1<ExprToken, ExprRule>;
    type TestProcessor = Processor<TestLang, TestLexer, TestParser>;

    TestProcessor::new()
        .build_lexer()?
        .build_parser()?
        .process::<SExp<_, _>>(input)
}

fn eval(ir: &SExp<'static, ExprToken, ExprRule>) -> i32 {
    macro_rules! match_atom {
        ($term:expr, $($kind:pat => $block:expr),* $(,)?) => {
            match $term {
                SExp::Atom(token) => {
                    match token.kind {
                        $($kind => $block,)*
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    match ir {
        SExp::List { rule, elems } => {
            match rule {
                ExprRule::Expr if elems.len() == 1 => eval(&elems[0]),
                ExprRule::Expr => {
                    let lhs = eval(&elems[0]);
                    let rhs = eval(&elems[2]);
                    match_atom!(elems[1],
                        ExprToken::Plus => lhs + rhs,
                        ExprToken::Minus => lhs - rhs,
                    )
                }
                ExprRule::Term if elems.len() == 1 => eval(&elems[0]),
                ExprRule::Term => {
                    let lhs = eval(&elems[0]);
                    let rhs = eval(&elems[2]);
                    match_atom!(elems[1],
                        ExprToken::Mul => lhs * rhs,
                        ExprToken::Div => lhs / rhs,
                    )
                }
                ExprRule::Num if elems.len() == 1 => eval(&elems[0]),
                ExprRule::Num => eval(&elems[1]),

            }
        }
        SExp::Atom(token) => token.as_str().parse().unwrap(),
    }
}
