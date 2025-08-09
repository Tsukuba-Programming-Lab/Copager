use copager::lang::Lang;
use copager::ir::r#ref::{CSTree, CSTreeWalker};

pub struct Top {}

impl<'input, L: Lang> From<CSTree<'input, L>> for Top {
    fn from(cst: CSTree<'input, L>) -> Self {
        let _ = CSTreeWalker::from(cst);
        Top {}
    }
}
