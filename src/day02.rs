use std::fs;
use std::error::Error;

type Move<'a> = (&'a str, i32);

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input02";
    let contents = fs::read_to_string(filename)?;
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

    Ok(())
}


fn solve02(_v: &Vec<Move>) -> i32 {
    let mut position = [0, 0];
    for line in _v {
        match line.0 {
            "down" => position[1] += line.1,
            "up" => position[1] -= line.1,
            "forward" => position[0] += line.1,
            _ => (),
        }
    }
    position[0] * position[1]
}

fn solve02_2(_v: &Vec<Move>) -> i32 {
    let mut position = [0, 0];
    let mut aim = 0;
    for line in _v {
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

