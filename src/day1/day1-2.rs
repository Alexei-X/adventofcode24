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
    split_in_tables(contents, &mut datas_table);
    let mut result = 0;
    for numb in datas_table.first_vec {
        let score = count_score(&mut datas_table.second_vec, &numb);
        result += score;
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

fn count_score(vec: &mut Vec<i32>, numb: &i32) -> i32 {
    let mut occ = 0;
    for n in vec {
        if n == numb {
            occ += 1;
        }
    }
    return occ*numb;
}
