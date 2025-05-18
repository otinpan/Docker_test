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

pub fn ensure_file_exists<P: AsRef<Path>>(file_path: P) -> Result<(), String> {
    let p = file_path.as_ref();

    match fs::metadata(p) {
        Ok(meta) if meta.is_file() => Ok(()),
        Ok(_)  => Err(format!("{} exists but is not a file", p.display())),
        Err(_) => Err(format!("{} not found", p.display())),
    }
}

pub fn write(dir_path:String,root:Root)->Result<(),Box<dyn Error>>{
    let path=Path::new(&dir_path);

    // ④ JSON 文字列に変換して保存
    let json = serde_json::to_string_pretty(&root)?;
    fs::write(path, json)?;

    Ok(())
}

