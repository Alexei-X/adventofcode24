use std::env;
use std::fs;

fn main() {
    let file_path: String = env::args().collect::<Vec<String>>()[1].clone();
    let contents = fs::read_to_string(file_path);
    let all_reports = split_reports(contents.expect("Should pass the content"));
    let mut result = 0;
    for report in all_reports {
        if is_safe(report.clone()) {
            result += 1;
        }
    }
    println!("{}", result);
}

fn split_reports(contents: String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut i = 0;
    for line in contents.lines() {
        result.push(vec![]);
        for c in line.split_whitespace() {
            result[i].push(c.parse::<i32>().unwrap());
        } 
        i += 1;
    }
    return result;
}

fn is_safe(report: Vec<i32>) -> bool {
    if report.len() < 2 {return true;}
    let growing: bool = (report[1] - report[0]) > 0;
    for i in 1..report.len() {
        let diff = report[i] - report[i-1];
        if (diff > 0) != growing || diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
    }
    return true;
}
