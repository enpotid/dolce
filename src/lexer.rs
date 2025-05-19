use crate::ast::*;

pub fn tokenize(src: String) -> Sheet {
    let src_iter = src.split("\n").map(|s| s.trim());

    let mut lines = Vec::new();
    let mut li = 0;
    for line in src_iter {
        li += 1;

        if line.trim() == "" {
            continue;
        }

        if line.len() < 2 {
            elog(li, "Each line must contain at least 2 characters.");
            continue;
        }

        match line {
            "|:" => lines.push(Line::Symbol(Symbol::SRepeat)),
            ":|" => lines.push(Line::Symbol(Symbol::ERepeat)),
            "D.C." => lines.push(Line::Symbol(Symbol::DaCapo)),
            "D.C. al Fine" => lines.push(Line::Symbol(Symbol::DaCapoAlFine)),
            "Fine" => lines.push(Line::Symbol(Symbol::Fine)),
            _ if line[..1] == *";" => {
                continue;
            }
            _ if line.len() > 4 && line[..4] == *"Dal " => {
                if line.len() > 12 && line[line.len() - 8..] == *" al Fine" {
                    lines.push(Line::Symbol(Symbol::DalAlFine(
                        line[4..line.len() - 8].to_string(),
                    )));
                } else {
                    lines.push(Line::Symbol(Symbol::Dal(line[4..].to_string())));
                }
            }
            _ if line.chars().last().unwrap() == ':' => {
                lines.push(Line::Symbol(Symbol::Label(
                    line[..line.len() - 1].to_string(),
                )));
            }
            _ => {
                let cdv: Vec<&str> = line.split("|").collect();
                if cdv.len() != 3 {
                    elog(li, "Example: A8, A3, Df5, Es1, C0 | 1000 | 100");
                    continue;
                }

                let mut e = false;
                let chord: Vec<Note> = cdv[0]
                    .split(",")
                    .map(|s| match Note::from(s.trim()) {
                        Some(s) => s,
                        None => {
                            elog(li, "All types of notes: C0, Cs0, Df0, D0, Ds0, Ef0, E0, F0, Fs0, Gf0, G0, Gs0, Af0, A0, As0, Bf0, B0, C1, Cs1, Df1, D1, Ds1, Ef1, E1, F1, Fs1, Gf1, G1, Gs1, Af1, A1, As1, Bf1, B1, C2, Cs2, Df2, D2, Ds2, Ef2, E2, F2, Fs2, Gf2, G2, Gs2, Af2, A2, As2, Bf2, B2, C3, Cs3, Df3, D3, Ds3, Ef3, E3, F3, Fs3, Gf3, G3, Gs3, Af3, A3, As3, Bf3, B3, C4, Cs4, Df4, D4, Ds4, Ef4, E4, F4, Fs4, Gf4, G4, Gs4, Af4, A4, As4, Bf4, B4, C5, Cs5, Df5, D5, Ds5, Ef5, E5, F5, Fs5, Gf5, G5, Gs5, Af5, A5, As5, Bf5, B5, C6, Cs6, Df6, D6, Ds6, Ef6, E6, F6, Fs6, Gf6, G6, Gs6, Af6, A6, As6, Bf6, B6, C7, Cs7, Df7, D7, Ds7, Ef7, E7, F7, Fs7, Gf7, G7, Gs7, Af7, A7, As7, Bf7, B7, C8");
                            e = true;
                            Note::A0
                        }
                    })
                    .collect();
                if e {
                    continue;
                }

                let duration = match cdv[1].trim().parse::<Duration>() {
                    Ok(d) => d,
                    Err(_) => {
                        elog(
                            li,
                            &format!(
                                "The range of durationd is from {} to {}.",
                                Duration::MIN,
                                Duration::MAX
                            ),
                        );
                        continue;
                    }
                };

                let volume = match cdv[2].trim().parse::<Volume>() {
                    Ok(v) => v,
                    Err(_) => {
                        elog(
                            li,
                            &format!(
                                "The range of volume is from {} to {}.",
                                Volume::MIN,
                                Volume::MAX
                            ),
                        );
                        continue;
                    }
                };

                lines.push(Line::Sound(Sound {
                    chord,
                    duration,
                    volume,
                }));
            }
        }
    }

    Sheet { lines }
}

fn elog(line: usize, msg: &str) {
    println!("\x1b[31mLine {}:\x1b[0m\n {}", line, msg);
}
