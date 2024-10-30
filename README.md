# Rust-Learn
> 각 하단 아래에 Comment 형식으로 요약문 작성
* Manual
    ```bash
    cargo build
    cargo run --bin [bin_name]
    ```
# Bin Name list
1. Variables
2. Datatype
3. Operator
4. Function
5. Branches
6. Ownership
7. Thread

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