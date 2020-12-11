mod day_1;
mod day_10;
mod day_11;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, mode, type_file] => {
            if mode == "create" {
                create_day_aoc(type_file)?;
            } else {
                panic!("Specify a valid mode");
            }
        }
        [_, _] => panic!("Specify a file type"),
        _ => {
            day_1::day_1()?;
            day_2::day_2()?;
            day_3::day_3()?;
            day_4::day_4()?;
            day_5::day_5()?;
            day_6::day_6()?;
            day_7::day_7()?;
            day_8::day_8()?;
            day_9::day_9()?;
            day_10::day_10()?;
            day_11::day_11()?;
        }
    };
    Ok(())
}

fn create_day_aoc(type_file: &str) -> std::io::Result<()> {
    use std::fs;
    use std::io::Write;

    let day: &str = &format!("day_{}", fs::read_dir("./src").unwrap().count());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(format!("src/{day}.rs", day = day))?;

    let content = match type_file {
        "lines" => format!(
            r#"use std::fs::File;
use std::io::{{self, prelude::BufRead, BufReader}};

#[allow(dead_code)]
pub fn {day}() -> io::Result<()> {{
    let file = File::open("inputs/{day}")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .collect();
    Ok(())
}}"#,
            day = day
        ),
        "string" => format!(
            r#"use std::{{fs, io}};

#[allow(dead_code)]
pub fn {day}() -> io::Result<()> {{
    let content = fs::read_to_string("inputs/{day}")?;
    Ok(())
}}"#,
            day = day
        ),
        _ => panic!("Invalid type"),
    };

    file.write_all(content.as_bytes())?;

    Ok(())
}
