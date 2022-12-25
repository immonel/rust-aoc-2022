use std::fs::File;
use std::io::{self, Read};
use std::process;

const FILE_PATH: &str = "input.txt";

fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_input() -> String {
    let contents = read_file_contents(FILE_PATH);

    match contents {
        Err(error) => {
            println!("Failed to read {FILE_PATH}:\n{error}");
            process::exit(1);
        },
        Ok(str) => str
    }
}