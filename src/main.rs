use std::{
    fs,
    env,
};

fn main() {
    let content = fs::read_to_string(get_file_to_open())
        .expect("Error while reading the file.")
        .chars()
        .filter(|x| ['.', ',', '<', '>', '[', ']', '+', '-'].iter().any(|i| x == i))
        .collect::<String>();

    let (mut pointer, mut arr_vals) = (0, [0]);

    println!("{}", content);
}

fn get_file_to_open() -> String {
    return String::from("E:\\Projets\\Langages\\Rust\\BrainfuckInterpreter\\bf\\main.bf")
    /*return if env::args().len() == 0 {
        String::from(format!("../bf/{}", "main.bf"))
    } else {
        format!("../bf/{}", env::args().collect::<Vec<String>>()[0].to_owned())
    }*/
}