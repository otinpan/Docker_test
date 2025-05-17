// main.rs
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path,PathBuf};
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use convert_to_json::{message,topic,test_case,Root};

fn main()  {

    let dir_path="./test1";

    let test_case=test_case::TestCase::new(&dir_path).unwrap_or_else(|err|{
        eprintln!("problem occured: {}",err);
        process::exit(1);
    });

    //println!("{:?}",test_case);

    //talk.txtからVec<Message>をつくる
    let messages=message::talk_converter(&test_case.talk).unwrap_or_else(|err|{
        eprintln!("application err in loading talk.txt: {}",err);
        process::exit(1);
    });

    //println!("{:?}",messages);

    //ans.txtからVec<Topic>をつくる
    let topics=topic::ans_converter(&test_case.answer).unwrap_or_else(|err|{
        eprintln!("apprication err in loading ans.txt: {}",err);
        process::exit(1);
    });

    //println!("{:?}",topics);
    // ③ JSON 構造体を作成
    let root = Root {
        messages,
        topics,
    };

    // ④ JSON 文字列に変換して保存
    let json = serde_json::to_string_pretty(&root).unwrap();
    fs::write("output.json", json).unwrap();

    println!("jsonファイルを生成しました。")

}



