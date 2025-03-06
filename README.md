# Rust Tutorials
Rust入门教程

## 通用知识
### 选择Rust不同版本
```
# 查看当前有哪些版本
rustup toolchain list

# 下载某个版本
rustup toolchain install 1.79

# 设置默认版本
rustup default 1.79
```

### dependencies和dev-dependencies
| 依赖类型                 | 何时编译                                                       | 何时链接          | 何时包含到最终产物 |
|----------------------|------------------------------------------------------------|---------------|-----------|
| `[dependencies]`     | `cargo build` <br/>`cargo test`                            | `cargo build` | ✅ 是的      |
| `[dev-dependencies]` | `cargo test` <br/>`cargo bench` <br/>`cargo run --example` | 仅在测试或基准测试时    | ❌ 否       |


### cargo build
* 用途：用于构建标准的`Rust`应用或库。
* 目标（Target）：编译为本机架构（如`x86_64`或`aarch64`）。
* 不适用于`Solana`智能合约，因为`Solana`运行的是`BPF/SBF`目标架构，而非本机架构。
### cargo build-bfp（旧版命令，已被build-sbf取代）
### cargo build-sbf
* 用途：用于编译 `Solana` 智能合约（以前的`Solana`运行时使用`BPF`，现已迁移到`SBF`）。
* 目标（Target）：`bpfel-unknown-unknown`（`Solana`的`BPF`运行时）。
* 输出文件：会生成 `.so` 共享库，存放在`target/deploy/`。

| 命令                 | 目标架构                    | 适用场景            | 备注                |
|--------------------|-------------------------|-----------------|-------------------|
| `cargo build`	     | 本机架构（`x86_64/aarch64`）  | 	`Rust`应用程序、库	  | 不能用于`Solana`合约    |
| `cargo build-bpf`	 | `bpfel-unknown-unknown` | 	旧版`Solana`智能合约 | 	已废弃，用`build-sbf` |
| `cargo build-sbf`	 | `sbf-solana-solana`	    | 新版`Solana`智能合约	 | 推荐使用              |



## 重要概念

### Trait
中文翻译为`特征、特性、特质`，类似其他语言的`接口`

### Iterator
迭代器是一种遍历集合（如数组、向量、哈希映射等）中元素的方法。通过使用迭代器，以一种高效且抽象的方式遍历集合中的元素，而不需要关心底层的数据结构。


## Time
* `Duration`：持续时间（一段时间）
* `Instant`：时刻（一个时间点）

### Chrono库
在`Rust`的`chrono`库中，`NaiveDate`、`NaiveTime`和`NaiveDateTime`是用于处理日期和时间的基本类型。它们被称为“朴素的”（`naive`），
因为这些类型不包含时区信息，适用于那些不需要考虑时区的场景。

* `NaiveDate`
  * 含义：表示一个没有时区的日期（即年-月-日）。
  * 用途：当你只需要处理日期本身而不关心具体的时间或时区时使用。例如，生日、节假日等只与日期有关的场景。
* `NaiveTime`
  * 含义：表示一个没有日期和时区的具体时间（即时:分:秒.纳秒）。
  * 用途：适合于仅需要关注时间的情况，如闹钟设置、工作时间等场景。
* `NaiveDateTime`
  * 含义：结合了`NaiveDate`和`NaiveTime`，代表一个没有时区的日期和时间。
  * 用途：当你需要同时记录日期和时间但又不需要考虑时区信息时非常有用。例如，记录某个事件发生的精确时刻（假设时区不影响该事件的记录）。

