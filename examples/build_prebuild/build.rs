use copager::Processor;

use language::Arithmetic;

type MyProcessor = Processor<Arithmetic>;

#[copager::prebuild]
fn main() -> MyProcessor {
    MyProcessor::new()
        .prebuild_parser()
        .unwrap()
}
