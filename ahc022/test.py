"""テストを並列実行する
"""

import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

import pandas as pd

BASE = Path(__file__).parent

SUBS_DIR = BASE / "subs"
TOOLS_DIR = BASE / "tools"
BINARY_DIR = BASE / "subs/target/release"
RESULT_FILE = BASE / "results"

THREAD_NUM = 10  # 並列実行を行うスレッドの数
TEST_NUM = 200  # 実行するテストケースの数

# データフレームのカラム名
DATAFRAME_COLS = [
    "L",
    "N",
    "S",
    "score",
    "wrong_answers",
    "placement_cost",
    "measurement_cost",
    "measurement_count",
]


def run_test(binary_name: int, testcase: int):
    """テストを実行する
    """
    # テストケースのファイル名
    input_file = f"{TOOLS_DIR}/in/{testcase:0>4}.txt"

    # 入力文字列を取得
    with open(input_file, mode="r") as f:
        input_text = f.read()
        input_text_byte = bytes(input_text, encoding="utf-8")
        # 入力の情報
        input_info = [int(v) for v in input_text.split("\n")[0].split()]

    # 実行ファイルのファイル名
    binary_file = BINARY_DIR / binary_name

    # テストケースを実行する
    completed = subprocess.run(
        ["cargo", "run", "--release", "--bin", "tester", binary_file],
        input=input_text_byte,
        cwd=TOOLS_DIR,
        capture_output=True,
        check=True
    )

    # 実行結果を表す文字列
    output = [int(line.split()[-1])
              for line in completed.stderr.decode().split("\n") if "=" in line]

    result = [
        *input_info,
        *output
    ]

    return result


def main():
    """並列でテストを実行する
    """
    # 実行するバイナリの名前
    binary_name = input("bin name: ")

    # ビルド
    subprocess.run(
        ["cargo", "build", "--release", "--bin", binary_name],
        cwd=SUBS_DIR,
        capture_output=True,
        check=True
    )

    # 結果を格納するデータフレーム
    dataframe = pd.DataFrame(columns=DATAFRAME_COLS,
                             index=range(TEST_NUM))

    # テストケースの実行
    with ThreadPoolExecutor(max_workers=THREAD_NUM) as executer:
        pool = {
            executer.submit(
                run_test,
                binary_name=binary_name,
                testcase=i,
            ): i for i in range(TEST_NUM)
        }
        # 結果の表示
        for res in as_completed(pool):
            testcase_id = pool[res]
            try:
                data = res.result()
            except Exception as exc:
                print(
                    f"testcase {testcase_id:0>4} generated an exception: {exc}")
            else:
                # データフレームに保存
                dataframe.loc[testcase_id] = data

    # 結果の出力
    print(dataframe)

    # スコアの合計
    print(f"score sum: {dataframe.score.sum():,}")

    # csvに保存
    dataframe.to_csv(RESULT_FILE / f"{binary_name}.csv")


if __name__ == "__main__":
    main()
