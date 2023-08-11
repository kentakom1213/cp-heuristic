#!/bin/zsh

## 全てのinputファイルをテストする
## `ahc018/`ディレクトリで実行する

sub1=$1
sub2=$2
bin1="../subs/target/release/$sub1"
bin2="../subs/target/release/$sub2"

# ビルド
cd ./subs
cargo build --release --bin $sub1 2> /dev/null
cargo build --release --bin $sub2 2> /dev/null

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
    local score1=$((cargo run --release --bin tester $bin1 < $input > /dev/null) |& awk 'NR==3 {print $4}')

    local score2=$((cargo run --release --bin tester $bin2 < $input > /dev/null) |& awk 'NR==3 {print $4}')

    cat $input | awk 'NR==1 {print "水源:", $2, ", 家:", $3, ", コスト:", $4}'
    echo "$score1 - $score2 = $(expr $score1 - $score2)\n"

    i=$((i + 1))
    sum=$(expr $score1 - $score2)
done

echo "SUM: $sum"

