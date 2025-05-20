use std::{collections::HashMap, process::exit};

use crate::ast::*;

pub fn parse(sheet: Sheet) -> Music {
    let mut labels = HashMap::new();
    let mut repeats = Vec::new();
    let mut lr = None;
    for i in 0..sheet.lines.len() {
        match &sheet.lines[i] {
            Line::Symbol(Symbol::Label(label)) => {
                let _ = labels.insert(label.clone(), i);
            }
            Line::Symbol(Symbol::SRepeat) => lr = Some(i),
            _ => {}
        }

        repeats.push(lr);
    }

    let mut sounds = Vec::new();
    parse_lines(
        &sheet,
        &mut sounds,
        &labels,
        &repeats,
        false,
        0,
        sheet.lines.len(),
    );

    Music { sounds }
}

fn parse_lines(
    sheet: &Sheet,
    sounds: &mut Vec<Sound>,
    labels: &HashMap<String, usize>,
    repeats: &Vec<Option<usize>>,
    fine: bool,
    s: usize,
    e: usize,
) -> bool {
    for i in s..e {
        match &sheet.lines[i] {
            Line::Sound(s) => sounds.push(s.clone()),
            Line::Symbol(s) => match s {
                Symbol::DaCapo => {
                    parse_lines(sheet, sounds, labels, repeats, false, 0, i);
                }
                Symbol::DaCapoAlFine => {
                    parse_lines(sheet, sounds, labels, repeats, true, 0, i);
                }
                Symbol::Dal(label) => {
                    if let Some(&li) = labels.get(label) {
                        if li > i {
                            elog(i + 1, "Can't find label earlier in the code.");
                            exit(1);
                        } else {
                            if parse_lines(sheet, sounds, labels, repeats, false, li, i) {
                                return true;
                            };
                        }
                    } else {
                        elog(i + 1, "Can't find label earlier in the code.");
                        exit(1);
                    }
                }
                Symbol::DalAlFine(label) => {
                    if let Some(&li) = labels.get(label) {
                        if li > i {
                            elog(i + 1, "Can't find label earlier in the code.");
                            exit(1);
                        } else {
                            if parse_lines(sheet, sounds, labels, repeats, true, li, i) {
                                return true;
                            };
                        }
                    } else {
                        elog(i + 1, "Can't find label earlier in the code.");
                        exit(1);
                    }
                }
                Symbol::SRepeat => {}
                Symbol::ERepeat => {
                    if let Some(sr) = repeats[i] {
                        if parse_lines(sheet, sounds, labels, repeats, false, sr, i) {
                            return true;
                        };
                    } else {
                        elog(i + 1, "Can't find '|:' earlier in the code.");
                        exit(1);
                    }
                }
                Symbol::Fine => {
                    if fine {
                        return false;
                    }
                }
                Symbol::Label(_) => {}
            },
        }

        if i == sheet.lines.len() - 1 {
            return true;
        }
    }

    false
}

fn elog(line: usize, msg: &str) {
    println!("\x1b[31mLine {}:\x1b[0m\n {}", line, msg);
}
