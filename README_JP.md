### **[English Version](/README.md)** 

# sfsync

**S**imple-**F**ile-**SYNC** software

## インストール方法

### ビルド済み実行ファイルをダウンロード

1. [Release Page](https://github.com/cykps/sfsync/releases) から自身の環境に合ったバイナリをダウンロード。
2. ダウンロードしたファイルの名前を`sfsync`、(Windowsの場合`sfsync.exe`)に変更。
3. 適切な場所に配置。(PATHの通った場所、共有したいフォルダなど)

### ソースコードからビルド

```
git clone git@github.com:cykps/sfsync.git
cd sfsync
cargo build
```

実行可能バイナリが`target/release`に生成されます。 

## 使い方

### サーバー側

ファイルを提供する側。

サーバー開始コマンド
```
$ sfsync --serve

### Windows
$ sfsync.exe --serve
```

ファイルを提供する間、常に起動しておく。

### クライアント側

ファイルを受け取る側。

ファイル取得コマンド
```
$ sfsync

### Windows
$ sfsync.exe
```

ファイルを取得したいときに都度実行する。

## VSCodeのLive Share拡張機能と併用する方法

### サーバー側

1. VSCodeで新しいターミナルを開き、`sfsync --serve`(もしくは`sfsync.exe --serve`)を常時実行しておく。
2. Live Share拡張機能でライブシェアを起動し、`左のバーのLive Share拡張機能のアイコン` -> `Shared Servers の中の Share server...` を順にクリック。
3. 出現した入力欄にポート番号(デフォルトでは`3000`)を入力。
4. 次にサーバーのニックネームを決める入力欄が現れるが、問題がなければそのまま`Enter`。
5. `Shared Servers`の欄にサーバーのニックネーム(デフォルトでは`localhost:[ポート番号]`)が表示されていることを確認する。

### クライアント側

1. 通常通り、Live Shareに参加する。
2. VSCodeでターミナルを開く。このときのターミナルはおそらくSharedターミナルになっているため、クライアント側から編集ができない。(もしSharedターミナルでなくローカルのターミナルとなっていたら3の工程は飛ばしても良い。)
3. 以下のいずれかの方法で新しいターミナルを開く。
    - ショートカットキー `Ctrl` + `Shift` + `&#96;` を使う。
    - ターミナルの右上の方の`+`ボタンを押す。
4. サーバー側から持ってきたファイルを保存するディレクトリに移動する。(`cd`コマンドなどで)
6. `sfsync`(もしくは`sfsync.exe --serve`)を実行することで、サーバーのファイルがローカルにコピーされる。
7. 任意のコマンドを実行 (例: `python3 main.py`)
8. コマンドの実行のたびに、6, 7を繰り返す

> [!TIP]
> コマンドを繋げる`&&`で繋げることで、6, 7を一つにまとめることができる。
> 例: `sfsync && python3 main.py`


## オプション

```
使用法: sfsync [オプション]

オプション:
  -s, --serve
  -p, --port <ポート番号>  [デフォルト: 3000]
  -h, --help         ヘルプを表示
  -V, --version      バージョンを表示
```

