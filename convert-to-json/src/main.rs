use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path,PathBuf};
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use convert_to_json::{message,topic,test_case,Root,write,ensure_file_exists};
use std::env;

fn main()  {

    let args:Vec<String>=env::args().collect();
    if args.len()<2{
        println!("not enough arguments");
        return
    }
    let dir_path=args[1].clone();


    //パスが存在するか調べる
    let test_case=test_case::TestCase::new(&dir_path).unwrap_or_else(|err|{
        println!("problem occured: {}",err);
        process::exit(1);
    });


    //jsonが存在するなら終了する
    if ensure_file_exists(&test_case.output).is_ok() {
        println!("file: {} exists ",test_case.output);
        process::exit(1);        // ② 見つかったら即終了 (exit code = 0)
    }


    //talk.txtからVec<Message>をつくる
    let messages=message::talk_converter(&test_case.talk).unwrap_or_else(|err|{
        println!("application err in loading talk.txt: {}",err);
        process::exit(1);
    });


    //ans.txtからVec<Topic>をつくる
    let topics=topic::ans_converter(&test_case.answer).unwrap_or_else(|err|{
        println!("apprication err in loading ans.txt: {}",err);
        process::exit(1);
    });

    //println!("{:?}",topics);
    // ③ JSON 構造体を作成
    let root = Root {
        messages,
        topics,
    };

    if let Err(e)=write(test_case.output,root){
        println!("write to folder error : {}",e);
        process::exit(1);
    }
    
    //すべて成功!
    println!("jsonファイルを生成しました。")

}



