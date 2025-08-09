use std::io::{stdin, Read};

use copager::template::LALR1;
use copager::ir::r#ref::{CSTree, CSTreeWalker};
use copager::Processor;

use example_lang_easyarith::ast::Top;
use example_lang_easyarith::eval::eval;
use example_lang_easyarith::syntax::EasyArith;

type Config = LALR1<EasyArith>;
type MyProcessor = Processor<Config>;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let cst = MyProcessor::new()
        .build()?
        .process::<CSTree<_>>(&input)?;
    let cst_walker = CSTreeWalker::from(cst);
    let ast = Top::from(cst_walker);
    eval(&ast);

    Ok(())
}
