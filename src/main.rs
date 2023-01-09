extern crate core;

use std::collections::HashMap;
use std::env;
use std::process;

fn main() {
    let limit = 4_000;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Program arguments are expected");
        process::exit(1);
    }

    let args: Vec<i32> = args.iter()
        .skip(1)
        .map(|arg| {
                match arg.trim().parse::<i32>() {
                    Ok(num) => {
                        if num >= limit {
                            eprintln!("Unexpected number: '{num}'. Value must be less then {limit}.");
                            process::exit(3);
                        }
                        num
                    },
                    Err(e) => {
                        eprintln!("Cannot parse input: '{}' – {}", arg, e);
                        process::exit(2);
                    },
                }
            })
        .collect();

    for num in args {
        let roman = convert(num);
        println!("Integer '{num}' converted to Roman '{roman}'.");
    }
}

fn convert(num: i32) -> String {
    let s = num.to_string();
    let abc: HashMap<u8, HashMap<u32, char>> = HashMap::from([
        (0, HashMap::from([
            (1, 'I'),
            (5, 'V'),
            (10, 'X')
        ])),
        (1, HashMap::from([
            (1, 'X'),
            (5, 'L'),
            (10, 'C')
        ])),
        (2, HashMap::from([
            (1, 'C'),
            (5, 'D'),
            (10, 'M')
        ])),
        (3, HashMap::from([
            (1, 'M')
        ]))
    ]);
    let mut result = String::new();
    let mut index = s.len()-1;
    for ch in s.chars() {
        let i = index as u8;
        let map = abc.get(&i);
        index = index.checked_sub(1).unwrap_or_default();
        let one = *map.unwrap().get(&1).unwrap();
        let five = *map.unwrap().get(&5).unwrap_or(&' ');
        let ten = *map.unwrap().get(&10).unwrap_or(&' ');
        let roman = match ch {
            '0' => String::new(),
            '1' => String::from(one),
            '2' => {
                let mut res = String::from(one);
                res.push(one);
                res
            },
            '3' => {
                let mut res = String::from(one);
                res.push(one);
                res.push(one);
                res
            },
            '4' => {
                let mut res = String::from(one);
                res.push(five);
                res
            },
            '5' => String::from(five),
            '6' => {
                let mut res = String::from(five);
                res.push(one);
                res
            },
            '7' => {
                let mut res = String::from(five);
                res.push(one);
                res.push(one);
                res
            },
            '8' => {
                let mut res = String::from(five);
                res.push(one);
                res.push(one);
                res.push(one);
                res
            },
            '9' => {
                let mut res = String::from(one);
                res.push(ten);
                res
            },
            _ => panic!("Unexpected symbol {ch} found while converting")
        };
        result.push_str(&roman);
    }
    result
}

#[test]
fn test3() {
    assert_eq!("III", convert(3))
}

#[test]
fn test10() {
    assert_eq!("X", convert(10))
}

#[test]
fn test57() {
    assert_eq!("LVII", convert(57))
}

#[test]
fn test1994() {
    assert_eq!("MCMXCIV", convert(1994))
}
