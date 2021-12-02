use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input08";
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

    println!("solve08: {:?}", solve08(&ints));
    println!("solve08_2: {:?}", solve08_2(&ints));

    Ok(())
}

fn solve08(_v: &Vec<i32>) {
}

fn solve08_2(_v: &Vec<i32>) {
}
