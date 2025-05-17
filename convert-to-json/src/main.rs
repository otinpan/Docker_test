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
use convert_to_json::{message,topic};

#[derive(Serialize, Deserialize)]
struct Root {
    messages: Vec<message::Message>,
    answer: Vec<topic::Topic>, // 今回は空で OK
}


#[derive(Debug)]
struct TestCase{
    pub talk:String,
    pub answer:String,
}

impl TestCase{
    fn new<P: AsRef<Path>>(dir:&P)->Result<TestCase,&'static str>{
        let path:&Path=dir.as_ref();
        let path_name=path.to_string_lossy().into_owned();
        //ファイルが存在するか調べる
        let entries=match fs::read_dir(dir){
            Ok(entries)=>entries,
            Err(_)=>return Err("not found folder "),
        };

        let talk=String::from("talk.txt");
        let answer=String::from("ans.txt");
        let mut count=0;
        let mut talk_counter=false;
        let mut ans_counter=false;
        //entriesはfs:read_dir(dir)の結果でディレクトリ内のファイルやサブディレクトリを1つずつ返すイテレータ
        for entry in entries {
            count+=1;
            if count>2{
                 return Err("this folder have too many files");
            }
            if let Ok(entry) = entry {
                if let Some(name) = entry.path().file_name() {
                    if name == "talk.txt" {
                        talk_counter=true;
                    }
                    if name == "ans.txt"{
                        ans_counter=true;
                    }
                }
            }
        }

        if !talk_counter{ 
            return Err("not found talk.txt");
        }
        if !ans_counter{
            return Err("not found ans.txt");
        }
        let talk=format!("{}/{}",path_name,talk);
        let answer=format!("{}/{}",path_name,answer);
        Ok(TestCase{talk,answer})
    }
}

fn main()  {

    let dir_path="./test1";

    let test_case=TestCase::new(&dir_path).unwrap_or_else(|err|{
        eprintln!("problem occured: {}",err);
        process::exit(1);
    });

    println!("{:?}",test_case);

    let messages=message::talk_converter(&test_case.talk).unwrap_or_else(|err|{
        eprintln!("application err: {}",err);
        process::exit(1);
    });

    println!("{:?}",messages);

    let topics=topic::ans_converter(&test_case.answer).unwrap_or_else(|err|{
        eprintln!("apprication err: {}",err);
        process::exit(1);
    });

    println!("{:?}",topics);
    // ③ JSON 構造体を作成
    let root = Root {
        messages,
        topics,
    };

    // ④ JSON 文字列に変換して保存
    let json = serde_json::to_string_pretty(&root).unwrap();
    fs::write("output.json", json).unwrap();

    println!("output.json を生成しました。");


}



