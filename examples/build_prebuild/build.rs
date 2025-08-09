use copager::template::LALR1;
use copager::Processor;

use language::Arithmetic;

type Config = LALR1<Arithmetic>;
type MyProcessor = Processor<Config>;

#[copager::prebuild]
fn main() -> MyProcessor {
    MyProcessor::new()
        .prebuild_parser()
        .unwrap()
}
