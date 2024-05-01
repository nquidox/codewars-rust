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

fn odd_count(n: u64) -> u64 {
    n/2
}

fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut res = Vec::new();
    let mut switch = true;
    for i in list{
        if i.to_string() != "flick" {
            res.push(switch);
        } else {
            switch = !switch;
            res.push(switch);
        }
    }
    res
}

fn main() {
    println!("Codewars");
    println!("{:?} | [false, true, false, true, false]", flick_switch(&["flick", "flick", "flick", "flick", "flick"]))
    // println!("{} | 7511", odd_count(15023))
    // println!("{:?}, [1,2,4,8,16]", powers_of_two(4));
    // println!("{}", reversed_strings("world"))
}