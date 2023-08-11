#!/bin/zsh

## 全てのinputファイルをテストする
## `ahc018/`ディレクトリで実行する

sub=$1
binary="../subs/target/release/$sub"

# ビルド
cd ./subs
cargo build --release --bin $sub 2> /dev/null

# # 実行
cd ../tools
# pbpaste | cargo run --release --bin tester $binary | pbcopy

# テストを実行
i=0
sum=0
for input in `ls ./in/*`
do
    # local out="./out/$input:t"
    # (cargo run --release --bin tester $binary < $input > /dev/null) |& awk 'NR==3 {print $4}'
    local score=$((cargo run --release --bin tester $binary < $input > /dev/null) |& awk 'NR==3 {print $4}')
    echo $score
    # sum=$((sum + score))
    # i=$((i + 1))
    # echo "$i: now=$sum"
done
