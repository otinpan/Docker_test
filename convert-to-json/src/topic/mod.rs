use regex::Regex;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Topic {
    pub topic_name: String,
    pub content: Vec<Example>,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Example {
    pub example: String,
}


pub fn ans_converter(ans_file:&str)->Result<Vec<Topic>,&'static str>{
    //ファイル読み込み
    let raw= fs::read_to_string(ans_file).map_err(|_| "failed to read file ans.txt")?;
    let quote_re = Regex::new(r#"^「(.+)」$"#).map_err(|_| "invalid regex ans.txt")?;
    let mut topics=Vec::new();
    let mut current:Option<Topic>=None;

     for line in raw.lines().filter(|l| !l.trim().is_empty()) {//空行は無視
       if let Some(caps) = quote_re.captures(line) {
            // 質問行
            let q = caps[1].to_string();
            if let Some(topic) = current.as_mut() {
                topic.content.push(Example { example: q }); //topic構造体のcontentフィールドにExsample型をpushした
            } else {
                return Err("there is no topic in ans.txt");
            }
        } else {
            // トピック行 → 直前のトピックを確定して次を開始
            if let Some(topic) = current.take() {
                topics.push(topic); //topicsにExsample型がいくつか入ったtopicをpushする
            }
            //新しいtopicをつくり、currentとする
            current = Some(Topic {
                topic_name: line.to_string(), 
                content: Vec::new(),
            });
        }
    }
    // 最終トピックの push
    if let Some(topic) = current {
        topics.push(topic);
    }
    return Ok(topics);
}