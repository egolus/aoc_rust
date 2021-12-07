use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input07";
    let contents = fs::read_to_string(filename)?;
    let v: Vec<&str> = contents.split(",").collect();
    let ints: Vec<i32> = v.iter().filter(|line| {
        match **line {
            "" => false,
            _ => true,
        }
    })
    .map(|line| {
        match line.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }).collect();

    let _tests = [16,1,2,0,4,2,7,1,2,14].to_vec();
    println!("solve07(tests): {}", solve07(&_tests));
    assert!(solve07(&_tests) == 37);
    println!("solve07: {:?}", solve07(&ints));
    println!("solve07_2(tests): {}", solve07_2(&_tests));
    assert!(solve07_2(&_tests) == 168);
    println!("solve07_2: {:?}", solve07_2(&ints));

    Ok(())
}

fn solve07(positions: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut sorted = positions.clone();
    sorted.sort();
    let target = sorted[sorted.len() / 2];
    for position in positions {
        result += (position - target).abs();
    }
    result
}

fn solve07_2(positions: &Vec<i32>) -> i32 {
    let mut result = 0;
    for target in 0..*positions.iter().max().unwrap() {
        let mut res = 0;
        for position in positions {
            let n = (position - target).abs();
            res += ((n * (n + 1)) / 2) as i32;
        }
        if result == 0 || res < result {
            result = res;
        }
    }
    result
}
