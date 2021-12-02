use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input05";
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

    println!("solve05: {:?}", solve05(&ints));
    println!("solve05_2: {:?}", solve05_2(&ints));

    Ok(())
}

fn solve05(_v: &Vec<i32>) {
}

fn solve05_2(_v: &Vec<i32>) {
}
