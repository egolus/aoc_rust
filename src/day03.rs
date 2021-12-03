use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input03";
    let contents = fs::read_to_string(filename)?;
    let v: Vec<&str> = contents.split("\n").filter(|line| {
        match *line {
            "" => false,
            _ => true,
        }
    }).collect();

    println!("solve03: {:?}", solve03(&v));
    println!("solve03_2: {:?}", solve03_2(contents.lines()));

    Ok(())
}

fn solve03(lines: &Vec<&str>) -> u32 {
    let length = lines[0].len();
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for i in 0..length {
        let mut b: u32 = 0;
        for line in lines {
            match line.chars().nth(i).expect("key error").to_digit(10) {
                Some(n) => b += n as u32,
                _ => ()
            }
        }
        b = (b as f32 / lines.len() as f32).round() as u32;
        println!("{}", b);
        match char::from_digit(b, 10) {
            Some('1') => {
                gamma.push('1');
                epsilon.push('0');
            },
            Some('0') => {
                gamma.push('0');
                epsilon.push('1');
            },
            Some(n) => println!("{}", n),
            _ => println!("not")
        };
    };
    println!("g: {}, e: {}", gamma, epsilon);
    u32::from_str_radix(gamma.as_str(), 2).unwrap() *
        u32::from_str_radix(epsilon.as_str(), 2).unwrap()
}

fn solve03_2(mut _v: std::str::Lines) {
}
