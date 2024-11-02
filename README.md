# Rust-Learn
> 각 하단 아래에 Comment 형식으로 요약문 작성
* Manual
    ```bash
    cargo build
    cargo run --bin [bin_name]
    ```
# Bin Name list
1. Datatype [Update : 24. 11. 02]
2. Operator [Update : 24. 10. 31]
3. Function [Update : 24. 11. 02]
4. Branches [Update : 24. 11. 02]
5. Ownership [Update : 24. 10. 30]
6. Variables [Update : 24. 10. 30]
7. Thread [Update : 24. 10. 30]

# Build Check
```bash
cargo clean
```

```bash
cargo build
```

```bash
time cargo check
```

# Toolchain Change
> Rust Toolchain 중 Stable, Beta, Nightly 가 설치되어 있어야 한다.

```bash
cargo +[version] test
```

```bash
cargo +stable test
cargo +beta test
cargo +nightly test
```

# Crate Lib Version Check
| Operator | EX      | Min      | Max     | Update Check |
|----------|---------|----------|---------|--------------|
| ^        | ^ 1.1.0 | >= 1.1.0 | < 2.0.0 | Yes          | 
| ~        | ~ 1.2.3 | >= 1.2.3 | < 1.3.0 | Yes          |