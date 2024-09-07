use grammar::MyProcessor;

#[copager::prebuild]
fn main() {
    let processor = MyProcessor::new();
}
