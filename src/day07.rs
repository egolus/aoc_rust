use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input07";
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

    println!("solve07: {:?}", solve07(&ints));
    println!("solve07_2: {:?}", solve07_2(&ints));

    Ok(())
}

fn solve07(_v: &Vec<i32>) {
}

fn solve07_2(_v: &Vec<i32>) {
}
