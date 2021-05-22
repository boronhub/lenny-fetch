use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

pub fn stdout_str(out: &Vec<u8>) -> String {
    String::from_utf8_lossy(out).to_string().trim().to_string()
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_split(txt: String, split: char, v: usize) -> String {
    let split: Vec<&str> = txt.split(split).collect();
    let name = split.get(v).unwrap();
    return name.to_string().trim().to_string();
}

pub fn pusher(obj: &(String, Option<String>), info: &mut Vec<String>) {
    if let Some(val) = &obj.1 {
        info.push(obj.0.to_owned() + &val);
    }
}