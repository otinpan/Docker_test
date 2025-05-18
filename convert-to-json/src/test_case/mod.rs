use std::fs;
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct TestCase{
    pub talk:String,
    pub answer:String,
}

impl TestCase{
    pub fn new<P: AsRef<Path>>(dir:&P)->Result<TestCase,&'static str>{
        let path:&Path=dir.as_ref();
        let path_name=path.to_string_lossy().into_owned();
        //ファイルが存在するか調べる
        let entries=match fs::read_dir(dir){
            Ok(entries)=>entries,
            Err(_)=>return Err("not found folder "),
        };

        let talk=String::from("talk.txt");
        let answer=String::from("ans.txt");
        let mut talk_counter=false;
        let mut ans_counter=false;
        //entriesはfs:read_dir(dir)の結果でディレクトリ内のファイルやサブディレクトリを1つずつ返すイテレータ
        for entry in entries {
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