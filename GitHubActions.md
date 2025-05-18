# GitHub Actions
GitHub ActionsとはGitHubがサービスの一環として提供する、ワークフロー自動化サービスである。

## 触れてみよう
実際にGitHub Actionsに触れてみる。ワークフローの作り方は2つあり、１つはGitHubのActionsを開くと出てくる、テンプレートを引っ張ることで作れる。2つ目は、自分で`.github/workflows`に`.yml`形式のファイルをつくり、動作を記述することである。今回は2つ目の方法で、ワークフローを実装する。
```yml
name: CI

# Controls when the workflow will run
on:
  # Triggers the workflow on push or pull request events but only for the "main" branch
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

  # Allows you to run this workflow manually from the Actions tab
  workflow_dispatch:


# the main part of workflow. This distribute action.
jobs:
  # job named "build"
  build:
    # configuration of virtualized environment
    runs-on: ubuntu-latest

    steps:
      # designate git hub actions varsion
      - name: Checkout
        uses: actions/checkout@v4

      # runs a single command
      - name: Run a one-line script
        run: echo Hello,world!
      
  # job named "hoge"
  hoge: 
    runs-on: ubuntu-latest
    steps:
      - name: Hoge step
        run: echo Hoge World!
```
インデントはスペースをミスすると動かないため気を付けよう！ymlファイルは大きく分けて2つのパートに分かれる
1. ワークフローのトリガーを記述する
2. ワークフローの中身を記述するパート  

### ワークフローのトリガーを記述するパート
```yml
name: CI

# Controls when the workflow will run
on:
  # Triggers the workflow on push or pull request events but only for the "main" branch
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

  # Allows you to run this workflow manually from the Actions tab
  workflow_dispatch:
```
まず`name:`でワークフローの名前をつける。`on:`では、どのタイミングでこのワークフローが走るかが定義されている。今回では
1. なんかしらが`main`ブランチに`push`されたとき
2. 新しく`pull_request`が`main`ブランチに作られたとき
3. ボタンが押されたとき(`workflow_dispatch:`) リポジトリの「Actions」タブ → 対象のワークフロー → 右上の「Run workflow」ボタン  


### ワークフローの中身を記述するパート
```yml
# the main part of workflow. This distribute action.
jobs:
  # job named "build"
  build:
    # configuration of virtualized environment
    runs-on: ubuntu-latest

    steps:
      # designate git hub actions varsion
      - name: Checkout
        uses: actions/checkout@v4

      # runs a single command
      - name: Run a one-line script
        run: echo Hello,world!
      
  # job named "hoge"
  hoge: 
    runs-on: ubuntu-latest
    steps:
      - name: Hoge step
        run: echo Hoge World!
```
`jobs:`では1つのワークフローの中に含まれる一連の処理を定義する。現在のワークフローには`build`と`hoge`が存在する。それぞれ、環境`runs-on:`と動作`steps:`が含まれている必要がある。
* `runs-on:`では、どのような仮想環境でジョブの処理を動かすかを定義する。今回は最新のubuntuを指定
* `steps:`では、具体的な一つ一つの処理を書く
    - 最初のステップでは`actions/checkout@v4`を指定する。これは自分のレポジトリのデータを読み込むという処理をする。今回は特にレポジトリのコードを使っているわけではないから、書かなくてもよい
    - 次はシェルスクリプトが走っている  


### workflow/.yml
```yml
name: Convert new folders in Test/text

on:
  push:
    branches: [feature]
    paths:
      - 'Test/text/**'

jobs:
  convert:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Detect added/changed folders in Test/text
        id: detect
        run: |
          # 変更されたファイルから親ディレクトリ名だけ抽出
          git diff --name-status HEAD^ HEAD | \
            grep '^A\|^M' | \
            awk '{print $2}' | \
            grep '^Test/text/' | \
            awk -F/ '{print $3}' | \
            sort | uniq > changed_dirs.txt

          echo "Changed dirs:"
          cat changed_dirs.txt

      - name: Run Rust conversion on each folder
        run: |
          while read dir; do
            echo "Processing folder: $dir"
            cargo run --release --manifest-path convert_to_json/Cargo.toml -- "Test/text/$dir"
          done < changed_dirs.txt

      - name: Commit and push JSON output
        run: |
          git config --global user.name 'github-actions'
          git config --global user.email 'github-actions@github.com'
          git add Test/json/
          git commit -m "Add JSON from pushed folder(s)"
          git push origin HEAD:${{ github.ref_name }}

```
### `main.rs`
```rust
use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

/// コマンドライン引数
#[derive(Parser)]
struct Args {
    /// 変換するディレクトリ (Test/text/<dir>)
    input_dir: PathBuf,
}

#[derive(Serialize)]
struct FileEntry {
    filename: String,
    content: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // ディレクトリ名だけ取り出して JSON 出力パスを決定
    let dir_name = args
        .input_dir
        .file_name()
        .context("Invalid directory")?
        .to_string_lossy();
    let out_path = PathBuf::from(format!("Test/json/{dir_name}.json"));

    // フォルダ配下を走査し .txt だけ拾う
    let mut entries = Vec::new();
    for entry in WalkDir::new(&args.input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("txt"))
    {
        let content = fs::read_to_string(entry.path())
            .with_context(|| format!("Reading {}", entry.path().display()))?;
        entries.push(FileEntry {
            filename: entry.file_name().to_string_lossy().into_owned(),
            content,
        });
    }

    // 必要ならここで解析・整形処理を行う
    // 例: CSV なら serde_csv で構造化するなど

    // JSON 書き出し先フォルダを確実に作る
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // pretty-print で保存
    fs::write(&out_path, serde_json::to_string_pretty(&entries)?)?;
    println!("Wrote {}", out_path.display());

    Ok(())
}
```

### 動作イメージ
```
Test/
└─ text/
   └─ fruit/          ← push されたフォルダ
      ├─ apple.txt
      └─ banana.txt
```
1. ワークフローが `cargo run -- "Test/text/fruit"` を実行

2. `fruit/` 内の `.txt` を読み取り

3. `Test/json/fruit.json` が生成され、内容例:
```json
[
  {
    "filename": "apple.txt",
    "content": "An apple a day keeps the doctor away."
  },
  {
    "filename": "banana.txt",
    "content": "Bananas are high in potassium."
  }
]
```
4. GitHub Actions が `git add Test/json/fruit.json` → コミット → プッシュ