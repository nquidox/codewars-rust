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

fn well(x: &[&str]) -> &'static str {
    let mut g_counter = 0;
    let mut b_counter = 0;

    for i in x{
        if i.to_string() == "good"{
            g_counter += 1;
        } else {
            b_counter += 1;
        }
    }

    if b_counter > 0 && g_counter == 0{
        return "Fail!"
    } else if g_counter > 0 && g_counter < 3{
        return "Publish!"
    } else if g_counter > 2 {
        return "I smell a series!"
    } else {
        return "Fail!"
    }
}

fn cockroach_speed(s: f64) -> i64 {
    return (s * 27.777777777777778) as i64
}

fn main() {
    println!("Codewars");
    println!("{} | 215", cockroach_speed(7.774066957199317))
    // println!("{} | I smell a series!", well(&["good", "bad", "bad", "bad", "bad", "good", "bad", "bad", "good"]))
    //println!("{:?} | [false, true, false, true, false]", flick_switch(&["flick", "flick", "flick", "flick", "flick"]))
    // println!("{} | 7511", odd_count(15023))
    // println!("{:?}, [1,2,4,8,16]", powers_of_two(4));
    // println!("{}", reversed_strings("world"))
}