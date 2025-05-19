use std::fs;
use std::path::{Path, PathBuf,Component};


#[derive(Debug)]
pub struct TestCase{
    pub talk:String,
    pub answer:String,
    pub output_distinguished:String,
    pub output_undistinguished:String,
    pub output_folder:String,
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

        let name:String;
        if let Some(file_name)=path.file_name(){
            name=file_name.to_string_lossy().to_string();
        }else{
            return Err("not make file name ");
        }

        
        let folder_name = match extract_folder_name(&path_name, 2) {
            Some(name) => name,
            None => return Err("cannot find folder "),
        };      
        


        let talk=format!("{}/{}",path_name,talk); //path_naem  ../Test/text/test1
        let answer=format!("{}/{}",path_name,answer);
        let output_distinguished=format!("{}/json/{}/distinguished.json",head_path_str(&path_name,2),name);
        let output_undistinguished=format!("{}/json/{}/undistinguished.json",head_path_str(&path_name,2),name);
        let output_folder=format!("{}/json/{}",head_path_str(&path_name,2),folder_name);

        //jsonが存在するなら終了する
        if ensure_directory_exists(&output_folder).is_ok() {
            return Err("json file is exist. you do not have to create json file");      // ② 見つかったら即終了 (exit code = 0)
        }

        Ok(TestCase{talk,answer,output_distinguished,output_undistinguished,output_folder})
    }
}


//先頭からn番目のフォルダ構成を出力　../Test/text/test1 2  -> ../Test
pub fn head_path_str<P: AsRef<Path>>(path: &P, n: usize) -> String {
    let head: PathBuf = path
        .as_ref()
        .components()
        .take(n)
        .fold(PathBuf::new(), |mut acc, c| {
            acc.push(match c {
                Component::Normal(s) => s,
                other => other.as_os_str(),
            });
            acc
        });

    head.to_string_lossy().into_owned()
}

// 指定した位置のフォルダの名前を返す
fn extract_folder_name<P: AsRef<Path>>(path: &P,pos :usize) -> Option<String> {
    let components: Vec<_> = path.as_ref().components()
        // Normal は普通のディレクトリやファイル名のコンポーネントを指します
        .filter_map(|c| match c {
            std::path::Component::Normal(os_str) => os_str.to_str(),
            _ => None,
        })
        .collect();

    // 今回の例で "test1" は3番目（インデックス2）にある想定
    // "../Test/json/test1/undistinguished.json" → ["..", "Test", "json", "test1", "undistinguished.json"]
    components.get(pos).map(|s| s.to_string())
}


//フォルダが存在するか調べる
pub fn ensure_directory_exists<P: AsRef<Path>>(dir_path: &P) -> Result<(), String> {
    let p = dir_path.as_ref();

    match fs::metadata(p) {
        Ok(meta) if meta.is_dir() => Ok(()),
        Ok(_) => Err(format!("{} exists but is not a directory", p.display())),
        Err(_) => Err(format!("{} not found", p.display())),
    }
}
