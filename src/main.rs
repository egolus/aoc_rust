use std::fs;

type Move<'a> = (&'a str, i32);

fn main() {
    day01();
    day02();
}

fn day01() {
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

fn day02() {
    let filename = "../input02";
    let contents = fs::read_to_string(filename)
        .expect("error reading file");
    let v: Vec<&str> = contents.split("\n").collect();
    let moves: Vec<Move> = v.iter().filter(|line| {
        match **line {
            "" => false,
            _ => true,
        }
    })
    .map(|line| line.split(" ").collect())
    .map(|line: Vec<&str>| {
        let m = line[0];
        let x = match line[1].parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        };
        (m, x)
    })
    .collect();

    println!("solve02: {:?}", solve02(&moves));
    println!("solve02_2: {:?}", solve02_2(&moves));
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

fn solve02(v: &Vec<Move>) -> i32 {
    let mut position = [0, 0];
    for line in v {
        match line.0 {
            "down" => position[1] += line.1,
            "up" => position[1] -= line.1,
            "forward" => position[0] += line.1,
            _ => (),
        }
    }
    position[0] * position[1]
}

fn solve02_2(v: &Vec<Move>) -> i32 {
    let mut position = [0, 0];
    let mut aim = 0;
    for line in v {
        match line.0 {
            "down" => aim += line.1,
            "up" => aim -= line.1,
            "forward" => {
                position[0] += line.1;
                position[1] += line.1 * aim;
            }
            _ => (),
        }
    }
    position[0] * position[1]
}
