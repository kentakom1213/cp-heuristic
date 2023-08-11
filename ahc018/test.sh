#!/bin/zsh

## テスト用スクリプト
## `ahc018/`ディレクトリで実行する

sub=$1
binary="../subs/target/release/$sub"

# ビルド
cd ./subs
cargo build --release --bin $sub 2> /dev/null

# 実行
cd ../tools
pbpaste | cargo run --release --bin tester $binary | pbcopy
