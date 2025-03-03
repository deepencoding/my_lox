pub mod init;
pub mod scanner;
pub mod tools;

use init::Lox;

fn main() {
    let mut lox = Lox::new();
    lox.start();
}