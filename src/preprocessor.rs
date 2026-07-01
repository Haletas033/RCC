use std::collections::{HashMap, HashSet};
use crate::preprocessor::Directive::*;

#[derive(Debug)]
pub enum Directive {
    Define { name: String, value: Option<String> },
    Undef { name: String },
    Ifdef,
    Ifndef,
    Endif,
    Else,
    Elif,
    Include,
    Pragma,
    Error,
    Line
}

pub struct Preprocessor {
    define_map: HashMap<String, Option<String>>,
    if_tree: Vec<bool>
}

impl Preprocessor {
    pub fn new() -> Preprocessor { Preprocessor{ define_map: HashMap::new(), if_tree: Vec::new() } }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '\'' || c == '\"'
    }
    fn keep_split(line: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut curr_token: String = String::new();
        let mut is_identifier: bool = true;
        for c in line.chars() {
            if Self::is_identifier_char(c) {
                if !is_identifier && !curr_token.is_empty() {
                    tokens.push(curr_token);
                    curr_token = String::new();
                }
                is_identifier = true;
                curr_token.push(c);
            } else {
                if is_identifier && !curr_token.is_empty() {
                    tokens.push(curr_token);
                    curr_token = String::new();
                }
                is_identifier = false;
                curr_token.push(c);
            }
        }
        tokens.push(curr_token);
        tokens
    }

    fn substitute_token(&self, token: &str, expanding: &mut HashSet<String>) -> Result<String, String> {
        if expanding.contains(token) { return Err(format!("Cannot resolve symbol \'{}\'", token).to_string()) }
        match self.define_map.get(token) {
            Some(Some(value)) => {
                expanding.insert(token.to_string());
                let values = Self::keep_split(value).iter()
                    .map(|token|{self.substitute_token(token, expanding)}).collect::<Result<Vec<String>, String>>()?.join("");
                Ok(values)
            },
            Some(None) => Ok(String::new()),
            None => Ok(token.to_string())
        }
    }

    fn substitute_line(&mut self, line: &str) -> Result<String, String> {
        let mut substituted_string: Vec<String> = Vec::new();
        let tokens = Self::keep_split(line);
        for token in tokens {
            substituted_string.push(self.substitute_token(&token, &mut HashSet::new())?);
        }
        Ok(substituted_string.join(""))
    }
    pub fn process(&mut self, file: &str) -> Result<String, String> {
        let mut output: String = String::new();
        let lines = file.lines();
        for line in lines {
            if line.is_empty() { continue }
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("//") { continue }

            if trimmed_line.starts_with('#') {
                let directive = get_directive(&trimmed_line)?;
                match directive {
                    Define { name, value } => {
                        self.define_map.insert(name, value);
                    },
                    Undef { name } => {
                        self.define_map.remove(&name);
                    }
                    _ => {},
                }
            } else {
                let substituted_line = self.substitute_line(trimmed_line)?;
                output.push_str(&substituted_line);
                output.push('\n');
            }
        }
        Ok(output)
    }
}

fn parse_identifier(name: &str) -> Result<(), String> {
    if name.starts_with('\'') || name.starts_with('\"') {
        return Err(
            format!("Macro name must be an identifier, got: \"{}\" instead.", name).to_string()
        );
    }
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => {
            return Err(format!(
                "Macro name must be an identifier, got: \"{}\" instead.",
                name
            ));
        }
    }

    if chars.any(|c| !(c.is_ascii_alphanumeric() || c == '_')) {
        return Err(format!(
            "Macro name must be an identifier, got: \"{}\" instead.",
            name
        ));
    }

    Ok(())
}

pub fn get_directive(line: &str) -> Result<Directive, String> {
    let clean_line = line.strip_prefix('#');

    let mut parts = clean_line.ok_or("Could not find # prefix on this line")?.split_whitespace();
    let directive_type = parts.next().ok_or("Failed to get the directive type")?;
    match directive_type {
        "define" => {
            let define_name = parts.next().ok_or("Failed to get define name")?;
            parse_identifier(define_name)?;

            let remaining: Vec<&str> = parts.collect();
            Ok(Define {
                name: define_name.to_string(),
                value: if remaining.is_empty() {
                    None
                } else {
                    Some(remaining.join(" "))
                },
            })
        },
        "undef" => {
            let undef_name = parts.next().ok_or("Failed to get define name")?;
            parse_identifier(undef_name)?;

            Ok(Undef {
                name: undef_name.to_string()
            })
        }
        _ => Err("Unsupported directive type".to_string())
    }
}