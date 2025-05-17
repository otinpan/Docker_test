use regex::Regex;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub fn talk_converter(talk_file: &str) -> Result<Vec<Message>, &'static str> {
    let raw = fs::read_to_string(talk_file).map_err(|_| "Failed to read file")?;

    // 正規表現: 例「A: こんにちは」「A：「こんにちは」」
    let re = Regex::new(
        r#"(?x)               # 拡張モード
        ^\s*                 # 行頭の空白
        ([A-Z])              # 話者（大文字1文字）
        \s*[:：]?\s*         # コロン（全角または半角）
        (?:「)?              # 開始のカギカッコ（任意）
        (.+?)                # 内容（最小一致）
        (?:」)?              # 終了のカギカッコ（任意）
        \s*$                 # 行末の空白
        "#
    ).map_err(|_| "Invalid regular expression")?;

    // 不要語削除（例：…や「笑」など）
    let remove_re = Regex::new(r"(…|（?笑）?)")
        .map_err(|_| "Invalid cleaning regular expression")?;

    let mut messages = Vec::new();

    for (i, line) in raw.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = re.captures(line) {
            let role = caps[1].to_string();
            let content = remove_re
                .replace_all(&caps[2], "")
                .trim()
                .to_string();
            messages.push(Message { role, content });
        } else {
            eprintln!("Warning: Invalid line at {}: '{}'", i + 1, line);
            return Err("Invalid line format");
        }
    }

    Ok(messages)
}
