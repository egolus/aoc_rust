use std::fs;

fn main() {
    let filename = "../input01";
    let contents = fs::read_to_string(filename)
        .expect("error reading file");
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
}

fn solve01(v: &Vec<i32>) -> i32 {
    let mut old = 0;
    let mut counter = -1;

    for n in v {
        if *n > old {
            counter += 1;
        }
        old = *n;
    }
    counter
}

fn solve01_2(v: &Vec<i32>) -> i32 {
    let mut old = 0;
    let mut counter = -1;
    for vals in v.windows(3) {
        let s: i32 = vals.iter().sum();
        if s > old {
            counter += 1;
        }
        old = s;
    }
    counter
}
