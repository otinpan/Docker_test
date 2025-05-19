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
use std::borrow::Cow;

pub mod message;
pub mod topic;
pub mod test_case;


#[derive(Serialize, Deserialize, Debug)]
pub struct Root{
    pub messages: Vec<message::Message>,
    pub answer:   Vec<topic::Topic>,
}


#[derive(Serialize, Deserialize,Debug)]
pub struct UndistinguishedRoot{
    pub messages: message::UndistinguishedMessage,
    pub answer: Vec<topic::Topic>,
}


//dir_pathのファイルにjsonを書き込む　../Test/json/test1/distinguished.json
pub fn write_json<P, S>(folder_path: &P,json_path: &P, root_data: &S) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    S: Serialize,
{
    //../Test/json/testの中にフォルダを作る
    fs::create_dir_all(folder_path);

    let path = json_path.as_ref();

    // pretty‑printed JSON にして書き込む
    let json = serde_json::to_string_pretty(root_data)?;
    fs::write(path, json)?;

    Ok(())
}
