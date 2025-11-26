use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("remove file error");
    println! ("file has been removed");
}