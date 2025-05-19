use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path,PathBuf};
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use convert_to_json::{message,topic,test_case,Root,UndistinguishedRoot,
    write_json};
use std::env;

fn main()  {
    //もしTest/Distinguished/text/... が変更されるならTestCase::newを変更してね
    let args:Vec<String>=env::args().collect();
    if args.len()<2{
        println!("not enough arguments");
        return
    }
    let dir_path=args[1].clone();


    //パスが存在するか調べる
    let test_case=test_case::TestCase::new(&dir_path).unwrap_or_else(|err|{
        println!("problem occured: {}",err);
        if err=="json file is exist. you do not have to create json file"{
            process::exit(0);
        }else{
            process::exit(1);
        }
    });

    println!("{:?}",test_case);

    //talk.txtからVec<Message>をつくる
    let messages=message::talk_converter(&test_case.talk).unwrap_or_else(|err|{
        println!("application err in loading talk.txt: {}",err);
        process::exit(1);
    });

    //ans.txtからVec<Topic>をつくる
    let answer=topic::ans_converter(&test_case.answer).unwrap_or_else(|err|{
        println!("apprication err in loading ans.txt: {}",err);
        process::exit(1);
    });

    let answer_copied=answer.clone();
    let messages_copied=messages.clone();

    // ③ JSON 構造体を作成
    let root = Root {
        messages,
        answer:answer,
    };

    if let Err(e)=write_json(&test_case.output_folder,&test_case.output_distinguished,&root){
        println!("write to folder error : {}",e);
        process::exit(1);
    }


    // undistinguished をつくる
    let messages=message::distinguished_to_undistinguished(&messages_copied).unwrap_or_else(|err|{
        println!("failed to distinguished message to undistinguished: {}",err);
        process::exit(1);
    });

    let root=UndistinguishedRoot{
        messages,
        answer: answer_copied,
    };

    if let Err(e)=write_json(&test_case.output_folder,&test_case.output_undistinguished,&root){
        println!("write to folder error : {}",e);
        process::exit(1);
    }



    
    //すべて成功!
    println!("jsonファイルを生成しました!")

}



