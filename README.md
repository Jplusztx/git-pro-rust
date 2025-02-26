# Git-Pro

Git-Pro 是一个用 Rust 编写的命令行工具，旨在简化日常 Git 操作。它提供了更简洁的命令接口，让 Git 操作更加直观和高效。

## 功能特点

- 一键提交所有更改
- 查看提交历史
- 修改最近一次提交
- 分支管理（创建、删除、重命名等）

## 安装

```bash
# 克隆仓库
git clone https://github.com/your-username/git-pro.git

# 进入项目目录
cd git-pro

# 编译安装
cargo install --path .
```

## 使用方法

### 提交更改

```bash
# 一键提交所有更改
git-pro commit -m "commit message"
```

### 查看提交历史

```bash
# 查看最近 10 条提交（默认）
git-pro log

# 查看指定数量的提交
git-pro log 5
```

### 修改最近提交

```bash
# 修改最近一次提交的信息
git-pro recommit -m "new commit message"
```

### 分支管理

```bash
# 列出所有分支
git-pro branch

# 创建新分支
git-pro branch new feature-1

# 基于指定分支创建新分支
git-pro branch new feature-2 -b main

# 删除分支
git-pro branch del feature-1

# 使用正则表达式删除多个分支
git-pro branch del-regex "^feature-.*"

# 强制删除匹配的分支（包括受保护分支）
git-pro branch del-regex -f "^test-.*"

# 重命名分支
git-pro branch rename old-name new-name
```

## 依赖

- Rust 1.70 或更高版本
- git2-rs
- clap
- thiserror
- anyhow

## 开发

```bash
# 构建项目
cargo build

# 运行项目
cargo run -- [command] [options]
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
