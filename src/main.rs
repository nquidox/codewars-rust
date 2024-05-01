#![allow(dead_code)]
fn reversed_strings(phrase: &str) -> String {
    return phrase.chars().rev().collect()
}

fn powers_of_two(n: u8) -> Vec<u128> {
    let mut res = Vec::new();
    for i in 0..n+1{
        res.push(2_u128.pow(i as u32));
    }
    res
}

fn main() {
    println!("Codewars");
    println!("{:?}, [1,2,4,8,16]", powers_of_two(4));
    // println!("{}", reversed_strings("world"))
}