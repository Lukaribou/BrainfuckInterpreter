fn main() {
    let content = std::fs::read_to_string(get_file_to_open())
        .expect("Error while reading the file.")
        .chars()
        .filter(|x| ['.', ',', '<', '>', '[', ']', '+', '-'].iter().any(|i| x == i))
        .collect::<Vec<char>>();

    let (mut pos, mut pointer, mut arr_vals) = (0, 0, vec![0 as u8]);
    let mut loops_index: Vec<usize> = vec![];

    while pos != content.len() {
        match content[pos] {
            '+' => { arr_vals[pointer] = if arr_vals[pointer] == 255 { 0 } else { arr_vals[pointer] + 1 } }
            '-' => { arr_vals[pointer] = if arr_vals[pointer] == 0 { 255 } else { arr_vals[pointer] - 1 } }
            '>' => { pointer += 1; if arr_vals.len() <= pointer { arr_vals.push(0) } }
            '<' => { pointer -= if pointer == 0 { 0 } else { 1 } }
            '.' => { print!("{}", arr_vals[pointer] as u8 as char) }
            '[' => { loops_index.push(pos) }
            ']' => {
                if arr_vals[pointer] != 0 { pos = *loops_index.last().expect("") }
                else { let _ = loops_index.pop().expect(""); } }
            ',' => { // Non fonctionnel, ne semble pas passer à la suite
                let input = String::from("");
                std::io::stdin().read_line(&mut String::from(input.trim())).expect("");
                if !input.is_ascii() { panic!("Input only accepts ASCII characters.") }
                if input.parse::<u8>().is_ok() { arr_vals[pointer] = input.parse::<u8>().unwrap() }
                else {
                    if input.len() != 1 { panic!("Input only accept one character or numbers lower than 256.") }
                    arr_vals[pointer] = input.chars().collect::<Vec<char>>()[0] as u8;
                }
            }
            _ => {}
        }
        pos += 1;
    }
}

fn get_file_to_open() -> String {
    return String::from("E:\\Projets\\Langages\\Rust\\brainfuck_interpreter\\bf\\main.bf")
    /*return if std::env::args().len() == 0 {
        String::from(format!("../bf/{}", "main.bf"))
    } else {
        format!("../bf/{}", std::env::args().collect::<Vec<String>>()[0].to_owned())
    }*/
}