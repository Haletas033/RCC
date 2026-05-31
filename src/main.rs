use crate::preprocessor::{Preprocessor};

mod preprocessor;

fn main() {
    let mut processor: Preprocessor = Preprocessor::new();
    println!("{:?}", processor.process("#define 'A' 42

int main() {
	'A'
}"));
}
