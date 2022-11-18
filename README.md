# [WIP] ya-fiat

This is simple tool for checking how much fiat money do you have on [yagna](https://github.com/golemfactory/yagna) account.

## Prerequisites
- [yagna](https://github.com/golemfactory/yagna) daemon up and running
- [Rust toolchain installed](https://www.rust-lang.org/tools/install)
- Linux distro

## How to install

Run following command inside this directory:
```sh
cargo install --path .
```

## How to use


This program reads yagna-specific data from stdin.

If you would like to see how much `usd` do you have, just type:

```sh
yagna payment status --json --network mainnet | ya-fiat usd
```

You will see following output with correct amount:
```
You have 21.37 usd
```
