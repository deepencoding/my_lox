use scanner::init::Lox;

pub mod scanner;

fn main() {
    let mut lox = Lox::new();
    lox.start();
}