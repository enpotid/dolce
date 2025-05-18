use crate::ast::*;

pub fn tokenize(src: String) -> Sheet {
    let src_iter = src.split("\n").map(|s| s.trim());

    let mut lines = Vec::new();
    for line in src_iter {
        if line.len() < 2 {
            continue;
        }

        match line {
            "|:" => {
                lines.push(Line::Symbol(Symbol::SRepeat));
            }
            ":|" => {
                lines.push(Line::Symbol(Symbol::ERepeat));
            }
            "D.C." => {
                lines.push(Line::Symbol(Symbol::DaCapo));
            }
            "Fine" => {
                lines.push(Line::Symbol(Symbol::Fine));
            }
            _ if line.len() > 4 && line[..4] == *"Dal " => {
                lines.push(Line::Symbol(Symbol::Dal(line[4..].to_string())));
            }
            _ if line.chars().last().unwrap() == ':' => {
                lines.push(Line::Symbol(Symbol::Label(
                    line[..line.len() - 1].to_string(),
                )));
            }
            _ => {
                let ctv: Vec<&str> = line.split("|").collect();
                if ctv.len() != 3 {
                    continue;
                }
            }
        }
    }

    Sheet { lines }
}
