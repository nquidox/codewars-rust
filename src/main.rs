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

    return if b_counter > 0 && g_counter == 0 {
        "Fail!"
    } else if g_counter > 0 && g_counter < 3 {
        "Publish!"
    } else if g_counter > 2 {
        "I smell a series!"
    } else {
        "Fail!"
    }
}

fn cockroach_speed(s: f64) -> i64 {
    return (s * 27.777777777777778) as i64
}

fn remove_exclamation_marks(input: &str) -> String {
    input.chars().filter(|c| c.to_string() != "!").collect()
}

fn pillars(num_of_pillars: u32, distance: u32, width: u32) -> u32 {
    return if num_of_pillars == 1 {
        0
    } else {
        (num_of_pillars - 2) * width + distance * (num_of_pillars - 1) * 100
    }
}

fn invert(values: &[i32]) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in values{
        new_vec.push(-i)
    }
    return new_vec
}

fn string_to_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn number_to_string(i: i32) -> String {
    i.to_string()
}

fn maps(values: &Vec<i32>) -> Vec<i32> {
    values.iter().map(|x| x*2).collect()
}

fn main() {
    println!("Codewars");
    println!("{:?} | [2, 4, 6, 8]", maps(&vec![1, 2, 3, 4]))
    // println!("{} | 1234", number_to_string(1234))
    // println!("{} | 1234", string_to_number("1234"));
    // println!("{:?} | [-1,-2,-3,-4,-5]", invert(&vec![1,2,3,4,5]));
    // println!("{} | 15270", pillars(11, 15, 30));
    // println!("{} | Hello World", remove_exclamation_marks("Hello World!!!"));
    // println!("{} | 215", cockroach_speed(7.774066957199317));
    // println!("{} | I smell a series!", well(&["good", "bad", "bad", "bad", "bad", "good", "bad", "bad", "good"]));
    //println!("{:?} | [false, true, false, true, false]", flick_switch(&["flick", "flick", "flick", "flick", "flick"]));
    // println!("{} | 7511", odd_count(15023));
    // println!("{:?}, [1,2,4,8,16]", powers_of_two(4));
    // println!("{}", reversed_strings("world"));
}