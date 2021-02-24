# 円周率の極み

なるべく多くの桁で円周率を計算したい.

## ディレクトリ構成

```
.
├── README.md
├── c
│   ├── fact.c
│   ├── fft.c
│   ├── mp_int.c
│   ├── pi.c
│   ├── sub.c
│   └── test_gmp.c
├── cpp
│   ├── Makefile
│   ├── bsm.cpp
│   ├── bsm_divide.cpp
│   ├── bsm_enhanced.cpp
│   ├── bsm_recursive.cpp
│   └── moyashi.cpp
└── rust
    └── pi
        ├── Cargo.lock
        ├── Cargo.toml
        ├── test.md(threadとかのテスト結果)
        ├── src
        │   ├── bigint.rs
        │   ├── bsm.rs
        │   ├── lib.rs
        │   ├── main.rs
        │   ├── multiprec
        │   │   ├── biguint.rs
        │   │   ├── fft.rs
        │   │   └── mod.rs
        │   ├── ramanujan.rs
        │   ├── thread-opt.rs
        │   └── thread.rs
```

### 進捗

#### 2020/12/26

- C++による Binary Splitting Method を用いた円周率計算
  - boost を利用
  - 今現在再帰を用いて実装しているため 1 万桁までのみ計算

#### 2020/12/28

- Binary Splitting Method の見直し(以前左から右に順に演算していた<-定義を見間違えていた)
  - tuple を用いずに struct で x, y, z の情報を持つことに
  - `n = N / 14`を導入(1 項計算するのに 14 桁分正しいので`N / 14`項計算ればいいことがわかる)
  - 1 億桁計算(大体 2 分ちょっとで)

#### 2021/01/15

- Rust で Binary Splitting Method 実装(`main.rs`)
- Rust でスレッディング(`thread_opt.rs`, `thread.rs`)

#### 2021/02/12

- ラマヌジャン実装(1 億桁だと落ちる, 原因検証まだ)
- Zenn 記事([Rust を使って円周率 1 億桁計算したお話](https://zenn.dev/uu/articles/48e2d4098b6aca))書いた

#### 2021/02/21

- Rust で FFT 実装

#### 2021/02/25

- BigUInt型の策定, multiprecモジュール作成

### コンパイル

#### C++

bsm_divide 以外のファイル用の Makefile は適宜作って.

```
make bsm_divide
```

#### Rust

```
cargo build --bin (main|thread(-opt)?|ramanujan|bsm) --release
```
