use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input06";
    let contents = fs::read_to_string(filename)?;
    let v: Vec<&str> = contents.split("\n").collect();
    let line_split = v[0].split(",");
    let _ints: Vec<i32> = line_split.map(|line| {
        match line.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }).collect();
    let _tests = [3, 4, 3, 1, 2].to_vec();

    assert!(solve06(&_tests, 80) == 5934);
    println!("solve06: {:?}", solve06(&_ints, 80));
    assert!(solve06(&_tests, 256) == 26984457539);
    println!("solve06_2: {:?}", solve06(&_ints, 256));

    Ok(())
}

fn solve06(input: &Vec<i32>, days: i32) -> i64{
    let mut result = 0;
    let mut fishtimes: HashMap<i32, i64> = HashMap::new();
    for i in 0..8 {
        fishtimes.insert(i, 0);
    }
    for fish in input {
        let entry = fishtimes.entry(*fish).or_insert(0);
        *entry += 1;
    }
    for day in 0..days {
        // reset current fish to 6
        *fishtimes.entry((day - 1).rem_euclid(8)).or_insert(0) += fishtimes[&(day.rem_euclid(8))];
        // add new fish with 8
        *fishtimes.entry((day + 1).rem_euclid(8) + 8).or_insert(0) += fishtimes[&(day.rem_euclid(8))];
        // get fishes of the next week
        *fishtimes.entry(day.rem_euclid(8)).or_insert(0) = match fishtimes.remove(&((day.rem_euclid(8)) + 8)) {
            Some(n) => n,
            _ => 0,
        };
        //println!("{}: {:?}", day, fishtimes);
    }
    for (_, v) in fishtimes.drain() {
        result += v;
    }
    result
}
