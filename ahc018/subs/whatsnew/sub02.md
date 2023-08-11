# sub02

少しだけ改良

## 方針
- ベースは貪欲法
- `C`（掘削一回あたりのコスト）に応じて変化をつける
  - `C`が小さいとき：掘削回数はあまり気にせず、無駄を減らす
  - `C`が大きいとき：一回のパワーを上げて、回数を減らす

### 計算式
- $P$：一回の掘削のパワー

$$
P = \max
\left\{
\begin{array}{l}
  50\\
  2C
\end{array}
\right.
$$