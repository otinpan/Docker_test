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



