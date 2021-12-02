use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input11";
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

    println!("solve11: {:?}", solve11(&ints));
    println!("solve11_2: {:?}", solve11_2(&ints));

    Ok(())
}

fn solve11(_v: &Vec<i32>) {
}

fn solve11_2(_v: &Vec<i32>) {
}
