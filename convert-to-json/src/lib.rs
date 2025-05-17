use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path,PathBuf};
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub mod message;
pub mod topic;
pub mod test_case;


#[derive(Serialize, Deserialize,Debug)]
pub struct Root {
    pub messages: Vec<message::Message>,
    pub topics: Vec<topic::Topic>, 
}


pub fn output(dir_path:String,root:Root)->Result<(),Box<dyn Error>>{
    let path=Path::new(&dir_path);

    let name:String;
    if let Some(file_name)=path.file_name(){
        name=file_name.to_string_lossy().to_string();
    }else{
        eprintln!("can't extract last part");
        process::exit(1);
    }
    let name=format!("{}_distinguished",name);
    let output_path=Path::new("../Test/json").join(format!("{}.json",name));
    // ④ JSON 文字列に変換して保存
    let json = serde_json::to_string_pretty(&root)?;
    fs::write(output_path, json)?;

    Ok(())
}