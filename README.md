# RUST 入门

## 0 安装 rustup
- 安装输入以下命令：

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

```

- 安装输入以下命令：

```
source ~/.bashrc
```


- 检查安装：

```
rustc --version
```

- 更新和卸载：

```
rustup update
rustup self uninstall
```

## 1 cargo 使用

```
cargo new <filepath>    # 新项目
cd <filepath>           # 进入项目
cargo check             # 确保可变翼
cargo run               # 构建并运行
cargo build             # 构建
cargo doc               # 生成文档
```


---

## Git

```
git add .
git commit -m "first commit"
git branch -M main
git push --set-upstream origin main
git remote add origin https://github.com/lancerstadium/rscode.git
git push -u origin main
```
