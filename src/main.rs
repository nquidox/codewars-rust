fn reversed_strings(phrase: &str) -> String {
    let mut res = String::new();
    for char in phrase.chars().rev(){
        res += &char.to_string();
    }
    return res
}

fn main() {
    println!("Codewars");
    println!("{}", reversed_strings("world"))
}