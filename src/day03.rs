use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input03";
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

    println!("solve03: {:?}", solve03(&ints));
    println!("solve03_2: {:?}", solve03_2(&ints));

    Ok(())
}

fn solve03(_v: &Vec<i32>) {
}

fn solve03_2(_v: &Vec<i32>) {
}
