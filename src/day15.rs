use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input15";
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

    println!("solve15: {:?}", solve15(&ints));
    println!("solve15_2: {:?}", solve15_2(&ints));

    Ok(())
}

fn solve15(_v: &Vec<i32>) {
}

fn solve15_2(_v: &Vec<i32>) {
}
