use std::io::{self, BufRead, Write};
use std::{collections::HashSet, fs::File};

pub fn get_sorted_lines(path: &str) -> String {
    // Read File
    let open_file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return "ERROR: FIle not found in".to_string() + path,
    };

    let reader = io::BufReader::new(open_file);
    let mut lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    // Sort Lines
    lines.sort();

    // Overwrite file (File::create trunc file if already exists)
    let write_file = match File::create(path) {
        Ok(f) => f,
        Err(_) => return "ERROR: Unable to write in the file ".to_string() + path,
    };

    let mut writer = io::BufWriter::new(write_file);

    for line in lines {
        if writeln!(writer, "{}", line).is_err() {
            return "ERROR: Failed while writing a line: ".to_string() + &line.to_string();
        }
    }

    "DONE".to_string()
}

pub fn merge_playlists(path_1: &str, path_2: &str) -> String {
    let open_file_1 = match File::open(path_1) {
        Ok(f) => f,
        Err(_) => return "ERROR: File 1 not found in".to_string() + path_1,
    };

    let open_file_2 = match File::open(path_2) {
        Ok(f) => f,
        Err(_) => return "ERROR: File 2 not found in".to_string() + path_2,
    };

    let reader_1 = io::BufReader::new(open_file_1);
    let mut lines_1: Vec<String> = reader_1.lines().map_while(Result::ok).collect();

    let reader_2 = io::BufReader::new(open_file_2);
    let mut lines_2: Vec<String> = reader_2.lines().map_while(Result::ok).collect();

    lines_1.append(&mut lines_2);

    // Overwrite or create file (File::create trunc file if already exists)
    let write_file = match File::create("./Merged_Playlist.txt") {
        Ok(f) => f,
        Err(_) => return "ERROR: Unable to write in the file ".to_string(),
    };

    let mut writer = io::BufWriter::new(write_file);

    for line in &lines_1 {
        if writeln!(writer, "{}", line).is_err() {
            return "ERROR: Failed while writing a line: ".to_string() + &line.to_string();
        }
    }

    remove_duplicated_lines(lines_1.clone());

    "DONE".to_string()
}

pub fn remove_duplicated_lines(mut lines:Vec<String>) -> String {
    let mut seen = HashSet::new();
    lines.retain(|x| seen.insert(x.clone()));
    println!("{:?}", lines);
    "DONE".to_string()
}
