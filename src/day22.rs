use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input22";
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

    println!("solve22: {:?}", solve22(&ints));
    println!("solve22_2: {:?}", solve22_2(&ints));

    Ok(())
}

fn solve22(_v: &Vec<i32>) {
}

fn solve22_2(_v: &Vec<i32>) {
}
