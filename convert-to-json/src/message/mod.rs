use regex::Regex;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub fn talk_converter(talk_file: &str) -> Result<Vec<Message>, &'static str> {
    let raw = fs::read_to_string(talk_file).map_err(|_| "failed to read file")?;

    let re = Regex::new(r"^([A-Z])「(.+)」$").map_err(|_| "invalid regex")?;
    let removed_word = vec!["…", "（笑）"];
    let mut messages = Vec::new();

    for line in raw.lines().filter(|l| !l.trim().is_empty()) {
        if let Some(caps) = re.captures(line) {
            let role = caps[1].to_string();
            let mut content = caps[2].to_string();
            for word in &removed_word {
                content = content.replace(word, "");
            }
            messages.push(Message { role, content });
        } else {
            return Err("invalid line");
        }
    }

    Ok(messages)
}
