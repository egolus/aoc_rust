use std::fs;
use std::error::Error;
use std::collections::HashMap;

type FromTo = (i32, i32, i32, i32);

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "../input05";
    let contents = fs::read_to_string(filename)?;
    let _test = "0,9 -> 5,9\n\
                 8,0 -> 0,8\n\
                 9,4 -> 3,4\n\
                 2,2 -> 2,1\n\
                 7,0 -> 7,4\n\
                 6,4 -> 2,0\n\
                 0,9 -> 2,9\n\
                 3,4 -> 1,4\n\
                 0,0 -> 8,8\n\
                 5,5 -> 8,2";
    //let v: Vec<&str> = _test.split("\n").collect();
    let v: Vec<&str> = contents.split("\n").collect();
    let lines: Vec<&str> = v.iter().filter(|line| **line != "").cloned().collect();
    let parsed_lines: Vec<FromTo> = parse_lines(&lines);

    println!("solve05: {:?}", solve05(&parsed_lines));
    println!("solve05_2: {:?}", solve05_2(&parsed_lines));

    Ok(())
}

fn parse_lines<'a>(lines: &Vec<&'a str>) -> Vec<FromTo>{
    lines.iter().map(|&line| {
        let mut split = line.split(" -> ");
        let fr = split.next().unwrap();
        let to = split.next().unwrap();
        let mut frs = fr.split(",");
        let mut tos = to.split(",");
        let x0 = frs.next().unwrap().parse::<i32>().expect("int");
        let y0 = frs.next().unwrap().parse::<i32>().expect("int");
        let x1 = tos.next().unwrap().parse::<i32>().expect("int");
        let y1 = tos.next().unwrap().parse::<i32>().expect("int");
        (x0, y0, x1, y1)
   }).collect()
}

fn solve05(lines: &Vec<FromTo>) -> i32 {
    let mut floor: HashMap<(i32, i32), i32> = HashMap::new();
    let mut result = 0;
    for line in lines {
        let (x1, y1, x2, y2) = line;
        if x1 != x2 && y1 != y2 {
            continue;
        }
        if x1 < x2 {
            for x in *x1..=*x2 {
                let entry = floor.entry((x, *y1)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        } else if x1 > x2 {
            for x in *x2..=*x1 {
                let entry = floor.entry((x, *y1)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        } else if y1 < y2 {
            for y in *y1..=*y2 {
                let entry = floor.entry((*x1, y)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        } else if y1 > y2 {
            for y in *y2..=*y1 {
                let entry = floor.entry((*x1, y)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        }
    }
    result
}

fn solve05_2(lines: &Vec<FromTo>) -> i32 {
    let mut floor: HashMap<(i32, i32), i32> = HashMap::new();
    let mut result = 0;
    for line in lines {
        //println!("{:?}", line);
        let (x1, y1, x2, y2) = line;
        if x1 != x2 && y1 != y2 {
            let mut xe = 1;
            let mut ye = 1;
            if x1 > x2 {
                xe = -1;
            }
            if y1 > y2 {
                ye = -1;
            }
            let mut x = *x1;
            let mut y = *y1;
            loop {
                let entry = floor.entry((x, y)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
                if x != *x2 {
                    x += xe;
                } else {
                    break;
                }
                if y != *y2 {
                    y += ye;
                } else {
                    break;
                }
            }
            continue;
        }
        if x1 < x2 {
            for x in *x1..=*x2 {
                let entry = floor.entry((x, *y1)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        } else if x1 > x2 {
            for x in *x2..=*x1 {
                let entry = floor.entry((x, *y1)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        } else if y1 < y2 {
            for y in *y1..=*y2 {
                let entry = floor.entry((*x1, y)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        } else if y1 > y2 {
            for y in *y2..=*y1 {
                let entry = floor.entry((*x1, y)).or_insert(0);
                *entry += 1;
                if *entry == 2 {
                    result += 1;
                }
            }
        }
    }
    result
}
