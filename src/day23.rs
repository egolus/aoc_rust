use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input23";
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

    println!("solve23: {:?}", solve23(&ints));
    println!("solve23_2: {:?}", solve23_2(&ints));

    Ok(())
}

fn solve23(_v: &Vec<i32>) {
}

fn solve23_2(_v: &Vec<i32>) {
}
