mod builder;
mod driver;

use serde::{Serialize, Deserialize};

use core::cfg::{TokenSet, Syntax};
use core::lex::Token;
use core::parse::ParserImpl;

use builder::LR1Configure;
use driver::LR1Driver;

#[derive(Debug, Serialize, Deserialize)]
pub struct LR1<'a, T, S> (LR1Configure<'a, T, S>)
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T>;

impl<'a, T, S> ParserImpl<'a> for LR1<'a, T, S>
where
    T: TokenSet<'a> + 'a,
    S: Syntax<'a, TokenSet = T>,
{
    type TokenSet = T;
    type Syntax = S;
    type Output = ();

    fn setup() -> anyhow::Result<Self> {
        Ok(LR1(LR1Configure::setup()?))
    }

    fn parse<'b>(
        &self,
        mut lexer: impl Iterator<Item = Token<'a, 'b, T>>,
    ) -> anyhow::Result<Self::Output> {
        LR1Driver::new(&self.0).run(&mut lexer)
    }
}

#[cfg(test)]
mod test {
    use core::cfg::{TokenSet, Syntax, Rule, RuleElem};
    use core::Parser;

    use super::LR1;

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TokenSet)]
    enum TestTokenSet {
        #[token(regex = r"\+")]
        Plus,
        #[token(regex = r"-")]
        Minus,
        #[token(regex = r"\*")]
        Mul,
        #[token(regex = r"/")]
        Div,
        #[token(regex = r"\(")]
        BracketL,
        #[token(regex = r"\)")]
        BracketR,
        #[token(regex = r"[1-9][0-9]*")]
        Num,
        #[token(regex = r"[ \t\n]+", ignored)]
        _Whitespace,
    }

    #[derive(Debug, Clone, Copy, Syntax)]
    enum TestSyntax {
        #[rule("<expr> ::= <expr> Plus <term>")]
        ExprPlus,
        #[rule("<expr> ::= <expr> Minus <term>")]
        ExprMinus,
        #[rule("<expr> ::= <term>")]
        ExprTerm,
        #[rule("<term> ::= <term> Mul <num>")]
        TermMul,
        #[rule("<term> ::= <term> Div <num>")]
        TermDiv,
        #[rule("<term> ::= <num>")]
        TermNum,
        #[rule("<num> ::= BracketL <expr> BracketR")]
        NestedNum,
        #[rule("<num> ::= Num")]
        Num,
    }

    #[test]
    fn input_ok() {
        let inputs = vec![
            "10",
            "10 + 20",
            "10 - 20",
            "10 * 20",
            "10 / 20",
            "10 + 20 * 30 - 40",
            "(10)",
            "((((10))))",
            "10 * (20 - 30)",
            "((10 + 20) * (30 / 40)) - 50",
        ];

        let parser = Parser::<LR1<TestTokenSet, TestSyntax>>::new().unwrap();
        for input in inputs {
            assert!(parser.parse(input).is_ok(), "{}", input);
        }
    }

    #[test]
    fn input_err() {
        let inputs = vec![
            "()",
            "(10 -",
            "10 +",
            "*",
            "10 20 + 30",
            "10 + 20 * 30 / 40 (",
            "(((10))",
        ];

        let parser = Parser::<LR1<TestTokenSet, TestSyntax>>::new().unwrap();
        for input in inputs {
            assert!(parser.parse(input).is_err(), "{}", input);
        }
    }
}
