fn reversed_strings(phrase: &str) -> String {
    return phrase.chars().rev().collect()
}

fn main() {
    println!("Codewars");
    println!("{}", reversed_strings("world"))
}