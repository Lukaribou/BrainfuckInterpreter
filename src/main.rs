use std::io::Write;

struct BF;
impl BF {
    const RIGHT: char = '>';
    const LEFT: char = '<';
    const INC: char = '+';
    const DEC: char = '-';
    const PRINT: char = '.';
    const INPUT: char = ',';
    const OPEN_LOOP: char = '[';
    const CLOSE_LOOP: char = ']';

    fn get_bf_tokens() -> Vec<char> {
        return vec![BF::RIGHT, BF::LEFT, BF::INC, BF::DEC, BF::PRINT, BF::INPUT, BF::OPEN_LOOP, BF::CLOSE_LOOP]
    }

    fn parse(string: &String) -> String {
        return string
            .chars()
            .filter(|x| BF::get_bf_tokens().iter().any(|i| x == i))
            .collect::<String>()
    }

    fn is_bf_file(path: &String) -> bool {
        return path.ends_with(".bf") || path.ends_with(".b")
    }

    fn run(content: &String) -> Result<String, &str> {
        let content = content.chars().collect::<Vec<char>>();
        let (mut pos, mut pointer, mut arr_vals) = (0, 0, vec![0 as u8]);
        let mut loops_index: Vec<usize> = vec![];
        let mut output: Vec<char> = vec![];

        while pos != content.len() {
            match content[pos] {
                BF::INC => { arr_vals[pointer] = if arr_vals[pointer] == 255 { 0 } else { arr_vals[pointer] + 1 } }
                BF::DEC => { arr_vals[pointer] = if arr_vals[pointer] == 0 { 255 } else { arr_vals[pointer] - 1 } }
                BF::RIGHT => { pointer += 1; if arr_vals.len() <= pointer { arr_vals.push(0) } }
                BF::LEFT => { pointer -= if pointer == 0 { 0 } else { 1 } }
                BF::PRINT => { output.push(arr_vals[pointer] as u8 as char) }
                BF::OPEN_LOOP => { loops_index.push(pos) }
                BF::CLOSE_LOOP => {
                    if arr_vals[pointer] != 0 { pos = *loops_index.last().expect("") }
                    else { let _ = loops_index.pop().expect(""); } }
                BF::INPUT => {
                    let mut input = String::from("");
                    print!("Input at position {}: ", pos.to_string());
                    let _ = std::io::stdout().flush();
                    std::io::stdin().read_line(&mut input).expect("");
                    input = input.drain(..input.len() - 1).collect();
                    if input.parse::<u8>().is_ok() { arr_vals[pointer] = input.parse::<u8>().expect("") }
                    else {
                        if input.len() != 1 { return Err("Input only accept one character or numbers lower than 256.") }
                        arr_vals[pointer] = input.chars().collect::<Vec<char>>()[0] as u8;
                    }
                }
                _ => {}
            }
            pos += 1;
        }
        Ok(output.into_iter().collect::<String>())
    }

    fn run_and_print(content: &String) -> Result<(), &str> {
        match BF::run(&content) {
            Ok(res) => { println!("{}", res); Ok(())},
            Err(err) => { Err(err) }
        }
    }
}

fn main() {
    let content = BF::parse(&std::fs::read_to_string(get_file_to_open())
        .expect("Error while reading the file."));

    BF::run_and_print(&content).expect("Error occured while running BF script ");
}

fn get_file_to_open() -> String {
    let path = /*if std::env::args().len() == 0 {
        String::from(format!("../bf/{}", "main.bf"))
    } else {
        format!("../bf/{}", std::env::args().collect::<Vec<String>>()[0].to_owned())
    }*/ String::from("E:\\Projets\\Langages\\Rust\\brainfuck_interpreter\\bf\\main.bf");
    return if BF::is_bf_file(&path) { path } else { panic!("File must be a .b/.bf file.") }
}