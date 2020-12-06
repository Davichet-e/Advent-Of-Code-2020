use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};

#[allow(dead_code)]
pub fn day_4() -> io::Result<()> {
    let mut file = File::open("inputs/day_4")?;
    let mut content = String::default();
    let re = Regex::new(r"((\S+):(\S+)(\n\n|\n$)?)").unwrap();
    file.read_to_string(&mut content)?;
    let captures: Vec<Captures> = re.captures_iter(&content).collect();
    // Part 1
    println!("Day 4\nPart 1: {}", check_passports(&captures, false));

    // Part 2
    println!("Part 2: {}\n", check_passports(&captures, true));

    Ok(())
}

#[derive(Default)]
struct Passport {
    fields: HashMap<String, String>,
}

fn validate_number(min: u16, max: u16, number: &str) -> bool {
    let number: Result<u16, _> = number.parse();
    match number {
        Ok(number) => number >= min && number <= max,
        _ => false,
    }
}

fn validate_field(field: (&str, &str)) -> bool {
    let value: &str = field.1;
    match field.0 {
        "hgt" => {
            lazy_static! {
                static ref HEIGHT_RE: Regex = Regex::new(r"(\d{2,3})(cm|in)").unwrap();
            }
            let cap = match HEIGHT_RE.captures(value) {
                None => return false,
                Some(v) => v,
            };
            let height = cap[1].parse::<u16>().unwrap();
            if &cap[2] == "in" {
                height >= 59 && height <= 76
            } else {
                height >= 150 && height <= 193
            }
        }
        "eyr" => validate_number(2020, 2030, value),
        "byr" => validate_number(1920, 2002, value),
        "iyr" => validate_number(2010, 2020, value),
        "pid" => {
            lazy_static! {
                static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            PID_RE.is_match(value)
        }
        "hcl" => {
            lazy_static! {
                static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            HAIR_RE.is_match(value)
        }
        "ecl" => {
            lazy_static! {
                static ref EYE_COLORS: HashSet<&'static str> =
                    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .into_iter()
                        .collect();
            }
            EYE_COLORS.contains(value)
        }
        "cid" => true,
        _ => unreachable!(),
    }
}

fn check_passports(captures: &[Captures], validations: bool) -> u16 {
    lazy_static! {
        static ref FIELDS: HashSet<&'static str> =
            vec!["hgt", "eyr", "hcl", "byr", "ecl", "pid", "iyr"]
                .into_iter()
                .collect();
    }
    captures
        .iter()
        .fold((0, Passport::default()), |(mut acc, mut passport), cap| {
            let c = String::from(&cap[2]);
            passport.fields.insert(c, String::from(&cap[3]));

            if cap.get(4).is_some() {
                if FIELDS.is_subset(
                    &passport
                        .fields
                        .keys()
                        .map(|r| &r[..])
                        .collect::<HashSet<&str>>(),
                ) && (!validations
                    || passport
                        .fields
                        .iter()
                        .all(|(key, value)| validate_field((&key, &value))))
                {
                    acc += 1;
                }
                passport = Passport::default();
            }
            (acc, passport)
        })
        .0
}
