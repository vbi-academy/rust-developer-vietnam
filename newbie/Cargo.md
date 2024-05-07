## Cargo trong Rust 

### Nó là gì ? 
Cargo là tool quản lý thư viện của Rust. Nó giúp cho bạn cài đặt thư viện 1 cách nhanh chóng, biên dịch code, public code trên crates.io

### cargo init 
Khởi tạo dự án Rust ban đầu 

Cú pháp cơ bản tạo `binary` package:

```bash
cargo init <tên project> 
```

Kết quả: tạo ra file `main.rs` và `Cargo.toml` -> `binary` package, nghĩa là tạo ra project chạy ứng dụng 

```
.
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```


Cú pháp cơ bản tạo `library` package:

```bash
cargo init --lib <tên project> 
```

Kết quả: tạo ra file `lib.rs` và `Cargo.toml` -> `library` package, nghĩa là tạo ra project dùng làm thư viện , người khác có thể sử dụng 

```
.
├── Cargo.toml
└── src
    └── lib.rs

2 directories, 2 files
```

### cargo build
Biên dịch code rust sang mã máy 

+ Biên dịch code với chế độ debug 

```bash
cargo build
```

+ Biên dịch code với chế độ optimized (chế độ chạy ứng dụng thực tế) 

```bash
cargo build --release 
```

### cargo check

Kiểm tra project có thể biên dịch hay không. 

```bash
cargo check
```

### cargo test 

Chạy các tests của dự án.
Lưu ý muốn viết test case thì cần có `#[cfg(test)]`

Ví dụ :

File `main.rs`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Viết test trong file `main.rs`:

```rust
// định nghĩa viết group tests 
#[cfg(test)]
// định nghĩa module test  
mod tests {
    // import tất cả các hàm public, struct, enum, biến toàn cục ,.. của file main.rs 
    use super::*;  

    // Viết unit test
    #[test]
    fn test_add() {
        // assertion
        assert_eq!(add(2, 2), 4);
    }
}
```


```bash
cargo test 
```

### cargo run  

Chạy dự án 

```bash
cargo run 
```

### cargo add  
Thêm thư viện bên ngoài vào dự án hiện tại. Các bạn có thể check các thư viện ở trên `crates.io`

Ví dụ : https://crates.io/crates/serde


```bash
cargo add <tên thư viện>  
```

### cargo tree
Hiển thị đồ thị phụ thuộc của thư viện (sub-dependencies)

Cú pháp:

```bash
cargo tree
```

```
├── log v0.4.21
├── serde v1.0.200
└── thiserror v1.0.59
    └── thiserror-impl v1.0.59 (proc-macro)
        ├── proc-macro2 v1.0.81
        │   └── unicode-ident v1.0.12
        ├── quote v1.0.36
        │   └── proc-macro2 v1.0.81 (*)
        └── syn v2.0.60
            ├── proc-macro2 v1.0.81 (*)
            ├── quote v1.0.36 (*)
            └── unicode-ident v1.0.12
```

## Tham khảo 
https://doc.rust-lang.org/cargo/








