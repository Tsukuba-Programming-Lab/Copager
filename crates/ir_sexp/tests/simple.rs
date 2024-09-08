use copager_cfg::token::TokenTag;
use copager_cfg::rule::{RuleTag, Rule, RuleElem};
use copager_lex::{LexSource, LexDriver};
use copager_lex_regex::RegexLexer;
use copager_parse::{ParseSource, ParseDriver, ParseEvent};
use copager_parse_lr1::LR1;
use copager_ir::{IR, IRBuilder};
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

type MyLexer = RegexLexer<ExprToken>;
type MyParser = LR1<ExprToken, ExprRule>;
type MyIR = SExp<'static, ExprToken, ExprRule>;

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
    let source = ExprToken::default();
    let lexer = <MyLexer as LexDriver<ExprToken>>::try_from(source).unwrap();

    let source = (ExprToken::default(), ExprRule::default());
    let parser = <MyParser as ParseDriver<ExprToken, ExprRule>>::try_from(source).unwrap();

    let mut ir_builder = <MyIR as IR<ExprToken, ExprRule>>::Builder::new();
    for event in parser.run(lexer.run(input)) {
        match event {
            ParseEvent::Read(token) => {
                ir_builder.on_read(token).unwrap();
            }
            ParseEvent::Parse { rule, len } => {
                ir_builder.on_parse(rule, len).unwrap();
            }
            ParseEvent::Err(err) => {
                return Err(anyhow::anyhow!("{:?}", err));
            }
        }
    }

    ir_builder.build()
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
