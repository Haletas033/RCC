use crate::preprocessor::{get_directive, Preprocessor};

mod preprocessor;

fn main() {
    let mut processor: Preprocessor = Preprocessor::new();
    println!("{:?}", processor.process("#define A B
#define B A

int main() {
	A B
}"));
}
