use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::{fs, io};

#[derive(Debug, Clone, Copy)]
struct Pair(u8, u8);

#[derive(Debug, Clone, Copy)]
struct Triplet(u8, u8, u8);

#[derive(Debug, Clone, Copy)]
enum Rule {
    Two(Pair),
    Or(Pair, Pair),
    OrOneTwo(u8, Pair),
    OrTwoThree(Pair, Triplet),
    OrOne(Pair),
    One(u8),
    Letter(char),
}

#[allow(dead_code)]
pub fn day_19() -> io::Result<()> {
    let content = fs::read_to_string("inputs/day_19")?;

    let mut a = content.split("\n\n");
    lazy_static! {
        static ref PAIR_OR_RE: Regex =
            Regex::new(r"(\d+): (\d+)? ?(\d+)? ?\|? ?(\d+)? ?(\d+)? ?(\d+)?").unwrap();
    }
    let mut memory: HashMap<u8, Rule> = HashMap::new();
    a.next().unwrap().lines().for_each(|l| {
        match PAIR_OR_RE
            .captures(l)
            .unwrap()
            .iter()
            .skip(1)
            .enumerate()
            .filter_map(|(i, c)| c.map(|m| (i, m.as_str().parse().unwrap())))
            .collect::<Vec<_>>()
            .as_slice()
        {
            [(_, key), (_, one), (2, two)] => {
                memory.insert(*key, Rule::Two(Pair(*one, *two)));
            }
            [(_, key), (_, one), (3, two)] => {
                memory.insert(*key, Rule::OrOne(Pair(*one, *two)));
            }
            [(_, key), (_, one), (_, two), (_, three)] => {
                memory.insert(*key, Rule::OrOneTwo(*one, Pair(*two, *three)));
            }
            [(_, key), (_, one)] => {
                memory.insert(*key, Rule::One(*one));
            }
            [(_, key), (_, one), (_, two), (_, three), (_, four)] => {
                memory.insert(*key, Rule::Or(Pair(*one, *two), Pair(*three, *four)));
            }
            [(_, key), (_, one), (_, two), (_, three), (_, four), (_, five)] => {
                memory.insert(
                    *key,
                    Rule::OrTwoThree(Pair(*one, *two), Triplet(*three, *four, *five)),
                );
            }
            [(_, key)] => {
                memory.insert(*key, Rule::Letter(if l.contains('b') { 'b' } else { 'a' }));
            }
            _ => unreachable!(),
        }
    });
    let p1 = solve(&memory);
    let solution: Regex = Regex::new(&format!("^{}$", p1)).unwrap();
    println!(
        "{}",
        a.next()
            .unwrap()
            .lines()
            .filter(|l| solution.is_match(*l))
            .count()
    );
    Ok(())
}

fn solve(memory: &HashMap<u8, Rule>) -> String {
    let mut desen = HashMap::<u8, String>::new();
    let mut s: Vec<(u8, Rule)> = Vec::new();
    let zero = if let Rule::Two(pair) = memory.get(&0).unwrap() {
        pair
    } else {
        unreachable!("{:?}", memory.get(&0).unwrap())
    };
    s.push((0, *memory.get(&0).unwrap()));
    s.push((zero.0, *memory.get(&zero.0).unwrap()));
    s.push((zero.1, *memory.get(&zero.1).unwrap()));
    while !s.is_empty() {
        match *s.last().unwrap() {
            (n, Rule::Letter(ch)) => {
                desen.entry(n).or_insert_with(|| ch.to_string());
                s.pop();
            }
            (n, Rule::Or(Pair(one, two), Pair(three, four))) => {
                if desen.contains_key(&n) {
                    s.pop();
                } else {
                    if desen.contains_key(&one) {
                        if desen.contains_key(&two) {
                            if desen.contains_key(&three) {
                                if desen.contains_key(&four) {
                                    desen.insert(
                                        n,
                                        format!(
                                            "(({}{})|({}{}))",
                                            desen.get(&one).unwrap(),
                                            desen.get(&two).unwrap(),
                                            desen.get(&three).unwrap(),
                                            desen.get(&four).unwrap()
                                        ),
                                    );
                                    s.pop();
                                } else {
                                    s.push((four, *memory.get(&four).unwrap()));
                                }
                            } else {
                                s.push((three, *memory.get(&three).unwrap()));
                            }
                        } else {
                            s.push((two, *memory.get(&two).unwrap()));
                        }
                    } else {
                        s.push((one, *memory.get(&one).unwrap()));
                    }
                }
            }
            (n, Rule::OrTwoThree(Pair(one, two), Triplet(_, _, _))) => {
                if desen.contains_key(&n) {
                    s.pop();
                } else {
                    if desen.contains_key(&one) {
                        if desen.contains_key(&two) {
                            desen.insert(
                                n,
                                format!(
                                    "({})",
                                    (1..5)
                                        .map(|i| {
                                            format!(
                                                "({}){{{}}}({}){{{}}}",
                                                desen.get(&one).unwrap(),
                                                i,
                                                desen.get(&two).unwrap(),
                                                i
                                            )
                                        })
                                        .join("|")
                                ),
                            );
                            s.pop();
                        } else {
                            s.push((two, *memory.get(&two).unwrap()));
                        }
                    } else {
                        s.push((one, *memory.get(&one).unwrap()));
                    }
                }
            }
            (n, Rule::OrOne(Pair(one, two))) => {
                if desen.contains_key(&n) {
                    s.pop();
                } else {
                    if desen.contains_key(&one) {
                        if desen.contains_key(&two) {
                            desen.insert(
                                n,
                                format!(
                                    "(({})|({}))",
                                    desen.get(&one).unwrap(),
                                    desen.get(&two).unwrap()
                                ),
                            );
                        } else {
                            s.push((two, *memory.get(&two).unwrap()));
                        }
                    } else {
                        s.push((one, *memory.get(&one).unwrap()));
                    }
                }
            }
            (n, Rule::OrOneTwo(one, Pair(_, _))) => {
                if desen.contains_key(&n) {
                    s.pop();
                } else {
                    if desen.contains_key(&one) {
                        desen.insert(n, format!("({})+", desen.get(&one).unwrap(),));
                    } else {
                        s.push((one, *memory.get(&one).unwrap()));
                    }
                }
            }
            (n, Rule::One(one)) => {
                if desen.contains_key(&n) {
                    s.pop();
                } else if desen.contains_key(&one) {
                    s.pop();
                    desen.insert(n, desen.get(&one).unwrap().clone());
                } else {
                    s.push((one, *memory.get(&one).unwrap()))
                }
            }
            (n, Rule::Two(Pair(one, two))) => {
                if desen.contains_key(&n) {
                    s.pop();
                } else if desen.contains_key(&one) {
                    if desen.contains_key(&two) {
                        desen.insert(
                            n,
                            format!("({}{})", desen.get(&one).unwrap(), desen.get(&two).unwrap()),
                        );
                        s.pop();
                    } else {
                        s.push((two, *memory.get(&two).unwrap()));
                    }
                } else {
                    s.push((one, *memory.get(&one).unwrap()));
                }
            }
        }
    }
    desen.get(&0).unwrap().clone()
}
