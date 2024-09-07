#![allow(dead_code)]

use std::collections::HashSet;

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

fn monkey_count(n: i32) -> Vec<i32> {
    (1..n+1).collect()
}

fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {"Even"} else {"Odd"}
}

fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().product()
}

fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    // let mut res = Vec::new();
    // let mut x = n;
    //
    // while x <= limit {
    //     res.push(x);
    //     x += n;
    // }
    //
    // res
    (n..=limit).step_by(n as usize).collect()
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty(){vec![]}
    else {
        vec![
        input.iter().filter(|x| x.is_positive()).count() as i32,
        input.iter().filter(|x| x.is_negative()).sum()
        ]
    }
}


fn to_alternating_case(s: &str) -> String {
    s.chars().map(|c| {if c.is_uppercase(){c.to_lowercase().to_string()} else {c.to_uppercase().to_string()}}).collect()
}

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x*x).sum()
}

fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    match nb_petals % 6 {
       1 => "I love you",
       2 => "a little",
       3 => "a lot",
       4 => "passionately",
       5 => "madly",
       _ => "not at all",
    }
}

fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&x| x).count() as u8
}

fn dna_strand(dna: &str) -> String {
    dna.chars().map(|c| match c {
        'A' => "T",
        'C' => "G",
        'T' => "A",
        'G' => "C",
        _ => panic!()
    }).collect()
}

fn find_short(s: &str) -> u32 {
    s.split_whitespace().map(|word| word.len()).min().unwrap_or(0) as u32
}

fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().map(|row| row.iter().min().unwrap()).sum()
}

fn sum_or_product(list: &[i64], n: usize) -> String {
    let mut sorted = list.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));

    let sum: i64 = sorted.iter().take(n).sum();
    let product: i64 = sorted.iter().skip(sorted.len()-n).product();

    if sum > product {"sum".to_string()} else if sum < product {"product".to_string()} else {"same".to_string()}
}

fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut sum = 0;
    for i in 0..arr_a.len(){
        if arr_a[i] == arr_b[i]{
            sum += 4
        } else if arr_b[i] == ""{
            continue
        } else {
            sum -= 1
        }
    }
    if sum > 0 { sum } else { 0 }
}

fn xo(string: &'static str) -> bool {
    string.chars().filter(|&c| c == 'x' || c == 'X').count() == string.chars().filter(|&c| c == 'o' || c == 'O').count()
}

fn square_area_to_circle(size:f64) -> f64 {
    std::f64::consts::PI*(size/4.0)
}

fn cook_pancakes(n: u32, m: u32) -> u32 {
    if n <= m { 2 } else { ((n as f64 * 2.0) / m as f64).ceil() as u32 }
}

fn get_sum(a: i64, b: i64) -> i64 {
    let mut sum = 0;
    let mut x = a;
    let mut y = b;
    if a > b {x = b; y = a}
    for i in x..=y {
        sum += i
    }
    sum
}

fn sequence(x: u8) -> Vec<u8> {
    let mut v:Vec<u8> = (1..=x).collect();
    v.sort_by_cached_key(|&x| x.to_string());
    v
}

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut res = Vec::new();

    for i in 2..=(integer/2){
        if integer % i == 0{
            res.push(i)
        }
    }

    if res.len() < 1 {
        return Err(format!("{integer} is prime"))
    }
    Ok(res)
}

fn ips_between(start: &str, end: &str) -> u32 {
    let s1: Vec<u32> = start.split(".").map(|x| x.parse().unwrap()).collect();
    let s2: Vec<u32> = end.split(".").map(|x| x.parse().unwrap()).collect();

    let mut res1 = 0;
    let mut res2 = 0;

    for i in 0..4{
        res1 = res1 * 256 + s1[i];
        res2 = res2 * 256 + s2[i];
    }
    res2 - res1
}

fn clean_string(s: &str) -> String {
    let mut res: Vec<char> = Vec::new();
    for c in s.chars(){
        match c {
            '#' => { res.pop(); },
            _ => res.push(c)
        }
    }
    res.into_iter().collect()
}

fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter().sum::<i32>() + ys.iter().sum::<i32>()
}

fn get_middle(s:&str) -> &str {
    let mid = s.len()/2;
    if s.len() % 2 == 0{
        &s[mid-1..mid+1]
    } else {
        &s[mid..mid+1]
    }
}

fn descending_order(x: u64) -> u64 {
    let mut v: Vec<u64> = x.to_string().chars().filter_map(|d| d.to_digit(10)).map(|d| d as u64).collect();
    v.sort_by(|a, b| b.cmp(a));
    v.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    match operator {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _ => panic!()
    }
}

fn frog_contest(n: u32) -> String {
    let chris = n * (n + 1) / 2;
    let tom = (chris/2) * (chris/2 + 1) / 2;
    let cat = (chris + tom) * (chris + tom + 1) / 2;
    format!("Chris ate {chris} flies, Tom ate {tom} flies and Cat ate {cat} flies")
}

fn is_pangram(s: &str) -> bool {
    let alphabet: HashSet<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let s_chars: HashSet<_> = s.to_lowercase().chars().collect();

    alphabet.is_subset(&s_chars)
}

fn gimme_the_letters(sp: &str) -> String {
    let range: Vec<&str> = sp.split("-").collect();
    let mut res = String::new();
    for i in range[0].chars().next().unwrap()..=range[1].chars().next().unwrap(){
        res += i.to_string().as_str()
    }
    res
}

fn get_char(c: i32) -> char {
    c as u8 as char
}

fn square_digits(num: u64) -> u64 {
    let squared: String = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d*d)
        .map(|d| d.to_string())
        .collect();
    squared.parse().unwrap()
}

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    mpg*gallons >= distance_to_pump
}

fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= dragons*2
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut r = p0;
    let mut years = 0;
    while r < p {
        r = r + (r as f64 *percent / 100.0) as i32 + aug;
        years += 1
    }
    years
}

fn symmetric_point(p: [i32; 2], q: [i32; 2]) -> [i32; 2] {
    let x1 = q[0] * 2 - p[0];
    let y1 = q[1] * 2 - p[1];
    [x1, y1]
}

fn rental_car_cost(d: u32) -> u32 {
    return if d < 3 {
        d * 40
    } else if d >= 3 && d < 7 {
        d * 40 - 20
    } else {
        d * 40 - 50
    }
}

fn move_hero(position: u32, roll: u32) -> u32 {
    position + roll*2
}

fn combat(health: f32, damage: f32) -> f32 {
    if health - damage > 0.0{
        health - damage
    } else {
        0.0
    }
}

fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    match (s1+s2+s3)/3 {
        90..=100 => 'A',
        80..90 => 'B',
        70..80 => 'C',
        60..70 => 'D',
        _ => 'F',
    }
}

fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *[a*(b+c), a*b*c, a+b+c, (a+b)*c].iter().max().unwrap()
}

fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}

fn difference_in_ages(ages: &[u8]) -> (u8, u8, u8) {
    (*ages.iter().min().unwrap(), *ages.iter().max().unwrap(), *ages.iter().max().unwrap() - ages.iter().min().unwrap())
}

fn nearest_sq(n: u32) -> u32 {
    let num= (n as f64).sqrt().round() as u32;
    num * num
}

fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = [arr1, arr2].concat();
    merged.sort();
    merged.dedup();
    merged
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("Codewars");
    println!("{:?} {:?}", merge_arrays(&[1,2,3,4], &[5,6,7,8]), &[1,2,3,4,5,6,7,8]);
    // println!("{} {}", nearest_sq(121), nearest_sq(111));
    // println!("{:?} {:?}", difference_in_ages(&[16, 22, 31, 44, 3, 38, 27, 41, 88]), (3, 88, 85))
    // println!("{} {}", make_upper_case("lowercase"), "LOWERCASE");
    // println!("{} {}", dna_to_rna("GCAT"), "GCAU")
    // println!("{} {}", expressions_matter(1, 2, 3), 9)
    // println!("{} {}", get_grade(95, 90, 93), "A")
    // println!("{} {}", find_smallest_int(&[34, -345, -1, 100]), -345);
    // println!("{} {}", combat(92.0, 8.0), 84.0)
    // println!("{} {}", move_hero(0, 4), 8);
    // println!("{} {}", rental_car_cost(4), 140);
    // println!("{} {}", rental_car_cost(8), 270);
    // println!("{:?} {:?}", symmetric_point([0, 0], [1, 1]), [2, 2])
    // println!("{} {}", nb_year(1500, 5.0, 100, 5000), 15);
    // println!("{} {}", hero(7, 4), false);
    // println!("{} {}", zero_fuel(50, 25, 2), true);
    // println!("{} {}", square_digits(9119), 811181);
    // println!("{} {}", get_char(64), '@');
    // println!("{} | hijklmno", gimme_the_letters("h-o"));
    // println!("{} | true", is_pangram("The quick, brown fox jumps over the lazy dog!"));
    // println!("{} | Chris ate 15 flies, Tom ate 28 flies and Cat ate 946 flies", frog_contest(5));
    // println!("{} | 11", basic_op('+', 4, 7));
    // println!("{} | 2110", descending_order(1021));
    // println!("{} | dd", get_middle("middle"));
    // println!("{} | 21", slice_plus_slice(&vec![1, 2, 3], &vec![4, 5, 6]));
    // println!("{} | ac", clean_string("abc#d##c"));
    // println!("{} | 50", ips_between("20.0.0.10", "20.0.1.0"));
    // println!("{:?} | [2, 3, 4, 6]", divisors(12));
    // println!("{:?} | [1, 10, 11, 12, 13, 14, 15, 16, 2, 3, 4, 5, 6, 7, 8, 9]", sequence(16));
    // println!("{} | 14", get_sum(-1, 5));
    // println!("{} | 3", cook_pancakes(3, 2));
    // println!("{} | 7.0685834705770345", square_area_to_circle(9.0));
    // println!("{} | false", xo("xxxm"));
    // println!("{} | 6", check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]));
    // println!("{} | same", sum_or_product(&[13, 8, 22, 39, 12, 6, 14, 19, 4, 7, 33], 4));
    // println!("{} | 16", sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 3]]));
    // println!("{} | 2", find_short("Let's travel abroad shall we"));
    // println!("{} | TAACG", dna_strand("ATTGC"));
    // println!("{} | 0", count_sheep(&[false]));
    // println!("{} | I love you", how_much_i_love_you(13));
    // println!("{} | 9", square_sum(vec![1, 2, 2]));
    // println!("{} | cODewARs", to_alternating_case("CodEWarS"));
    // println!("{:?} | [10, -65]", count_positives_sum_negatives(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15]));
    // println!("{:?} | [11, 22, 33, 44]", find_multiples(11, 54));
    // println!("{} | 16", grow(vec![4, 1, 1, 1, 4]));
    // println!("{} | Even", even_or_odd(2));
    // println!("{:?} | [1, 2, 3, 4, 5]", monkey_count(5));
    // println!("{:?} | [2, 4, 6, 8]", maps(&vec![1, 2, 3, 4]));
    // println!("{} | 1234", number_to_string(1234));
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