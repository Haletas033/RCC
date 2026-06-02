use crate::preprocessor::{Preprocessor};

mod preprocessor;

fn main() {
    let mut processor: Preprocessor = Preprocessor::new();
    println!("{:?}", processor.process("#define FOO 42

int main() {
	FOO
}"));
}
