# 基于 Cargo 的 SysY 编译器项目模板

该仓库中存放了一个基于 Cargo 的 SysY 编译器项目的模板, 你可以在该模板的基础上进行进一步的开发.

该仓库中的 Rust 代码实现仅作为演示, 不代表你的编译器必须以此方式实现. 如你需要使用该模板, 建议你删掉所有 Rust 源文件, 仅保留 `Cargo.toml` 和必要的目录结构, 然后重新开始实现.

事实上, 基于 Cargo 的编译器项目**并不需要模板**. 不过为了保持和 [sysy-make-template](https://github.com/pku-minic/sysy-make-template) 以及 [sysy-cmake-template](https://github.com/pku-minic/sysy-cmake-template) 的一致性, 此处还是开放一个模板项目吧.

你只需要使用如下命令:

```sh
cargo init --bin
```

即可在任意目录初始化一个基于 Cargo 的 Rust 项目, 该项目无需进行任何额外的修改, 即可被评测平台正确编译.

## 使用方法

**注意: 通常情况下, 建议你不要使用本模板, 而直接使用 `cargo init` 初始化 Cargo 项目.**

首先 clone 本仓库:

```sh
git clone https://github.com/pku-minic/sysy-cmake-template.git
```

进入仓库目录后执行:

```sh
cargo run -- -koopa input.c -o output.koopa
```

Cargo 将自动构建并运行该项目.

如在此基础上进行开发, 你需要重新初始化 Git 仓库:

```sh
rm -rf .git
git init
```

然后将自己的编译器的源文件放入 `src` 目录.

## 评测平台要求

当你提交一个根目录包含 `Cargo.toml` 文件的仓库时, 评测平台会使用如下命令编译你的编译器:

```sh
cargo build --manifest-path "Cargo.toml的路径" --release
```

默认情况下, 你无需进行任何更改, 只需要确保项目的 manifest 文件名为 `Cargo.toml` 即可.
