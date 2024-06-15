## 作用

方便地实现下面三个脚本的功能

```bash
git update-index --assume-unchanged README.md ## 假设某文件未变更，从而在git add时忽略它
git ls-files -v | grep '^[a-z]' ## 查看哪些文件被忽略了
git update-index --no-assume-unchanged README.md ## 取消假设某文件未变更
```

做这个工具的初衷，`.gitignore` 文件并不能达到这样的效果：保持某个文件为现状，对该文件的本地变更不会同步到远程仓库。这在本地测试一些用于vscode调试的配置文件时非常有用。经过搜索后发现，这样的效果需要用上面提到的命令来做。但是这样的命令不太方便，所以写了这个工具。

另外，观察到jetbrains的ide貌似内置了在 `.gitignore` 文件添加内容时，自动对其假设未变更的功能。

## 安装

```bash
cargo install --git https://github.com/arloor/repo
```

## 使用

```bash
repo list
repo miss .vscode/*
repo watch .vscode/launch.json
repo watch .vscode/launch.json
```