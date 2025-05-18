use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path,PathBuf};
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use convert_to_json::{message,topic,test_case,Root,output};
use std::env;

fn main()  {

    let args:Vec<String>=env::args().collect();
    if args.len()<2{
        eprintln!("not enough arguments");
    }
    let dir_path=args[1].clone();


    //パスが存在するか調べる
    let test_case=test_case::TestCase::new(&dir_path).unwrap_or_else(|err|{
        eprintln!("problem occured: {}",err);
        process::exit(1);
    });


    //talk.txtからVec<Message>をつくる
    let messages=message::talk_converter(&test_case.talk).unwrap_or_else(|err|{
        eprintln!("application err in loading talk.txt: {}",err);
        process::exit(1);
    });


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

    if let Err(e)=output(dir_path,root){
        eprintln!("output error : {}",e);
        process::exit(1);
    }
    
    //すべて成功!
    println!("jsonファイルを生成しました。")

}



