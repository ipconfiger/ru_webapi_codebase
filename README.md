# 自用项目模版，请谨慎使用!

## A：使用方法

### A1: 手动

在release中下载最新的代码，解压后，将文件夹重命名为项目名称

修改 Cargo.toml 中的 name 字段为项目名称

修改 Cargo.toml 中的 version 字段为项目版本号

修改 Cargo.toml 中的 authors 字段为项目作者

修改 Cargo.toml 中的 description 字段为项目描述

在终端进入项目目录，执行以下命令
```bash
git init
git add .
git commit -m "init"
git remote add origin <your-repo-url>
git push -u origin master
cargo build
```

如果编译成功，即可即刻进入项目开发阶段

### A2: 使用 cargo-generate

在终端执行以下命令
```bash
cargo install cargo-generate
cargo generate --git git@github.com:ipconfiger/ru_webapi_codebase.git --name <your-project-name>
cd <your-project-name>
cargo build
```
### A3：使用 ru-cpj
这是个自己顺手做的项目，起因是cargo-generate最新版本不支持stable的最新版rustc。

项目地址在： https://github.com/ipconfiger/ru-cpj

在终端执行命令
```bash
cargo install ru-cpj
ru-cpj <your-project-name> https://github.com/ipconfiger/ru_webapi_codebase/archive/refs/tags/0.1.1.zip
```



## B: 项目依赖

项目模版中使用了几个私有项目，如果不需要，可以删除 Cargo.toml 中的依赖项
### aliyun-sts-rust-sdk
这个项目是阿里云的 sts rust sdk，用于获取阿里云的临时访问凭证，用于访问阿里云的服务，因为阿里云本身没有提供Rust的SDK，所以自己实现了一个

### sqlx_struct_enhanced
这个项目是自己对sqlx的简单封装，主要是给struct注入了增删改查的方法，方便使用，主要针对自己的使用习惯，你可以删掉换上自己习惯的ORM框架

### rmmp
这是在同步模型设计到目标代码的生成器，项目在 https://github.com/ipconfiger/rmmp 中，主要用来将models目录下的模型定义同步生成数据库的models结构体，各种接口定义的结构体，返回数据结构的结构体，以及给前端访问接口定义的请求参数和返回值的结构体定义，和Rust代码同步，方便开发。同时也生成了结构体的类关系图，方便在文档中引用。
!!! 注意：生成UML图依赖了工具 dot_graph，你需要先自己安装

### C: 开发说明

项目模版定义了双层Router结构，方便拆分子项目。

example 为定义的模版子Router结构，最好不要修改

使用的时候使用在 scripts目录下的脚本 create.sh 来创建子Router结构，脚本会自动创建子Router结构。

```bash
./scripts/create.sh mod <your-sub-router-name>
```


