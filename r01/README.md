# r01

```bash
$ cargo new r01
warning: `/home/xxx/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
warning: `/home/xxx/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
    Creating binary (application) `hello_word` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# 消除 warning, 执行下面命令 & 直接将 /home/xxx/.cargo/config 命名为config.toml
$ ln -s /home/xxx/.cargo/config /home/xxx/.cargo/config.toml

$ rustc src/main.rs
$ ./main
Hello, world!

$ cargo run
```
