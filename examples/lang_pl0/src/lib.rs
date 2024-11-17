mod token;
mod rule;

use copager::cfl::CFL;
use copager::lex::RegexLexer;
use copager::parse::LR1;
use copager::Generator;

pub use token::Pl0Token;
pub use rule::Pl0Rule;

type Configure<T> = Generator<T, RegexLexer<T>, LR1<T>>;
pub type Pl0 = Configure<Pl0Lang>;

#[derive(Debug, Default, CFL)]
pub struct Pl0Lang (
    #[tokens] Pl0Token,
    #[rules]  Pl0Rule,
);
