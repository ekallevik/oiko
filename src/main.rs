use crate::parser::parse_csv;

mod parser;

fn main() {
    println!("Hello, world!");

    let transactions = parse_csv("./Siste transaksjoner.txt").unwrap();
    for transaction in transactions {
        println!("{}", transaction)
    }
}
