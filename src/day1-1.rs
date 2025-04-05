use std::env;
use std::fs;

struct Tables {
    first_vec: Vec<i32>,
    second_vec: Vec<i32>,
}

fn main() {
    let file_path: String = env::args().collect::<Vec<String>>()[1].clone();
    let mut datas_table = Tables {
        first_vec: Vec::new(),
        second_vec: Vec::new(),
    };
    let contents = fs::read_to_string(file_path)
        .expect("Error reading the file");
    let mut all_dist = Vec::<i32>::new();
    split_in_tables(contents, &mut datas_table);
    loop {
        if datas_table.first_vec.len() == 0 {
            break;
        }
        let first_min = find_min(&mut datas_table.first_vec);
        let second_min = find_min(&mut datas_table.second_vec);
        all_dist.push((first_min - second_min).abs());
    }
    let mut result = 0;
    for i in all_dist {
        result += i;
    }
    println!("{}", result);
}

fn split_in_tables(contents: String, datas_table: &mut Tables) {
    for line in contents.lines() {
        let mut i = 1;
        for c in line.split_whitespace() {
            if i%2 == 1 {
                datas_table.first_vec.push(c.parse::<i32>().unwrap());
                i += 1;
            } else {
                datas_table.second_vec.push(c.parse::<i32>().unwrap());
            }
        }
    }
}

fn find_min(vec: &mut Vec<i32>) -> i32 {
    let mut current_min: i32 = vec[0];
    let mut index = 0;
    for i in 1..vec.len() {
        if vec[i] < current_min {
            current_min = vec[i];
            index = i;
        } 
    }
    vec.remove(index);
    return current_min;
}
