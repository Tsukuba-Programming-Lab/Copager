use language::MyProcessor;

#[copager::prebuild]
fn main() -> MyProcessor {
    MyProcessor::new()
        .prebuild_parser()
        .unwrap()
}
