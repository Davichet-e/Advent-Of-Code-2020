import sys
import os

type_ = sys.argv[1]
n = len(os.listdir("src"))

with open(f"src\\day_{n}.rs", "w") as file:
    if type_ == "lines":
        file.write(
            f"""\
use std::fs::File;
use std::io::{{self, prelude::BufRead, BufReader}};

#[allow(dead_code)]
pub fn day_{n}() -> io::Result<()> {{
    let file = File::open("inputs/day_{n}")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .collect();
    Ok(())
}}
"""
        )

    elif type_ == string:
        file.write(
            f"""\
use std::{{fs, io}};

#[allow(dead_code)]
pub fn day_{n}() -> io::Result<()> {{
    let content = fs::read_to_string("inputs/day_{n}")?;
    Ok(())
}}
"""
        )

    else:
        raise Exception("Specify type")

