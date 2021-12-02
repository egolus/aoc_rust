use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input01";
    let contents = fs::read_to_string(filename)?;
    let v: Vec<&str> = contents.split("\n").collect();
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

    println!("solve01: {:?}", solve01(&ints));
    println!("solve01_2: {:?}", solve01_2(&ints));

    Ok(())
}

fn solve01(_v: &Vec<i32>) -> i32 {
    let mut old = 0;
    let mut counter = -1;

    for n in _v {
        if *n > old {
            counter += 1;
        }
        old = *n;
    }
    counter
}

fn solve01_2(_v: &Vec<i32>) -> i32 {
    let mut old = 0;
    let mut counter = -1;
    for vals in _v.windows(3) {
        let s: i32 = vals.iter().sum();
        if s > old {
            counter += 1;
        }
        old = s;
    }
    counter
}
