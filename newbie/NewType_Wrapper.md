## NewType Wrapper trong Rust

### Yêu cầu kiến thức
+ Hiểu cơ bản cách sử dụng và ứng dụng của `Trait`


### NewType Wrapper là gì 
`NewType Wrapper` là 1 pattern sử dụng phương pháp `wrap`(đóng gói) các kiểu dữ liệu có sẵn trong Rust, cho phép chúng ta có thể mở rộng nhiều chức năng hơn 

### Ví dụ cơ bản 


#### Giả sử in ra Vector String 
+ Ta thấy rằng Vector và String là 2 kiểu dữ liệu mà Rust cung cấp 
+ Khi in ra thì `trait Debug` đã implement cho Vector (https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-Debug-for-Vec%3CT,+A%3E)

```rust
fn main(){
    let vec = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
    println!("Vec:{:?}", vec);
}
```

Kết quả:
```
Vec:["One", "Two", "Three"]
```

#### Giờ mình muốn kết quả in ra là các phần tử phân tách nhau bằng dấu chấm phẩy 
Kết quả mình mong muốn:
```
Vec:["One"; "Two"; "Three"]
```

#### Làm sao nhỉ -> Implement trait Debug cho Vec<String> 

```rust
use std::fmt;

// Re-implement trait Debug cho Vec<String> 
impl fmt::Debug for Vec<String> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}]", self.join(", ")) }
	}

fn main(){
    let vec = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
    println!("Vec:{:?}", vec);
}
```

#### Rust không cho phép Re-implement Trait cho Vec<String>

```| impl fmt::Debug for Vec<String> {
   | ^^^^^^^^^^^^^^^^^^^^-----------
   | |                   |
   | |                   `Vec` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead
```

-> Vi phạm `Orphan Rule` (You can implement a trait for a type only if either the trait or the type is defined in your crate) 
-> Nghĩa là bạn có thể implement trait cho kiểu dữ liệu nếu trait hoặc kiểu dữ liệu đó đang định nghĩa ở module/crate của bạn 
-> Trong trường hợp trên
+ Debug : có import vào crate
+ Vec<T>: đây là private struct , nên chúng ta không thể import 

-> Suy ra vi phạm quy tắc `Orphan Rule` 


#### Cách giải quyết -> Sử dụng New Type Wrapper

```rust
use std::fmt;
// sử dụng kiểu dữ liệu mới -> wrap kiểu dữ liệu có sẵn 
// Wrapper là kiểu dữ liệu customized , mình có quyền extend chức năng và re-implement trait 
struct Wrapper(Vec<String>);
// Re-implement Debug for Wrapper 
impl fmt::Debug for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "Wrapper([")?;
        // Iterate từng phần tử trong Vector
        for (i, elem) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "; ")?;
            }
            write!(f, "\"{}\"", elem)?;
        }
        write!(f, "])")
    }
}

fn main() {
    let w = Wrapper(vec!["Hello".to_string(), "world".to_string()]);
    println!("{:?}", w);
}
```

Kết quả: 
```
Wrapper(["Hello"; "world"])
```

### Ví dụ thực tế 

#### Sử dụng newtype Wrapper với From/Into và TryFrom/TryInto 

Phần này mình đã giới thiệu trong phần `From_Into.md` 
[From_Into.md](From_Into.md)

#### Một số references 
1. https://paritytech.github.io/polkadot-sdk/master/sp_core/struct.H160.html

+ Chúng ta sẽ không hiểu ý nghĩa thực sự của `[u8;20]` để làm gì 

```rust
let array: [u8; 20] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    ];
```

+ Sử dụng new type wrapper cho mục đích là biểu diễn thông tin Hash của 1 address ETH 

```rust
pub struct H160(pub [u8;20]);
let array: [u8; 20] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    ];
let hash = H160(array);
```

+ Bài toán cơ bản: Convert String to H160 

Phần này mình có thể tự định nghĩa 1 newtype wrapper riêng hoặc có thể sử dụng một số crates như `sp-core`, ...

Step 0: Tạo project mới

```
cargo new --lib newtype
```

Step 1: Import `sp-core` (using cargo add sp-core)

Step 3: Implement và test convert String sang kiểu wrapper type 

```rust
use core::str::FromStr;
use sp_core::H160;

#[derive(Clone, Debug, PartialEq)]

pub enum ConvertH160Error {
    InvalidAddress(String),
}

pub fn is_valid_address(address: &str) -> Result<H160, ConvertH160Error> {
    H160::from_str(&address)
        .map(Into::into)
        .ok()
        .ok_or(ConvertH160Error::InvalidAddress(address.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_address() {
        let valid_address_str = "0x783FC27915754512E72b5811599504eCa458E4C5";
        assert!(is_valid_address(valid_address_str).is_ok());
    }

    #[test]
    fn test_is_not_valid_address() {
        let invalid_1 = "0x123";
        assert!(is_valid_address(invalid_1).is_err());
        let invalid_2 = "1223";
        assert!(is_valid_address(invalid_2).is_err());
        let invalid_3 = "i am a robot";
        assert!(is_valid_address(invalid_3).is_err());
        let invalid_4 = "0x783FC27915754512E72b5811599504eCa458E4C51012";
        assert!(is_valid_address(invalid_4).is_err());


    }
}
```

Run test case
```
cargo test 
```

Step 4: Mở rộng chức năng cho `H160` như initialize `0` , convert sang `bytes` ,... 


2. https://github.com/solana-labs/solana/blob/master/sdk/src/signer/keypair.rs#L25




### Ưu/Nhược điểm khi sử dụng NewType Wrapper

#### Ưu điểm 

##### 1. Type safety : Dữ liệu an toàn 

1. Vấn đề: Giả dụ mình tạo user với email và password. Đôi lúc mình sơ ý điền password trước , email sau. Khiến cho việc validate không an toàn.

```rust
pub fn create_user(email: &str, password: &str) -> Result<User, CreateUserError> {
    validate_email(email)?;  
    validate_password(password)?;
    let password_hash = hash_password(password)?;  
    // Save user to database  
    // Trigger welcome emails
    // ...
    Ok(User)  
}
```

2. Giải quyết: Constraint biến đầu vaò bắt buộc là phải Wrapper Email và Wrapper Password -> `Thông tin đầu vào rõ ràng và đảm bảo an toàn dữ liệu`

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress(String);  
 
#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);
 
pub fn create_user(email: EmailAddress, password: Password) -> Result<User, CreateUserError> {
    validate_email(&email)?;  
    validate_password(&password)?;  
    let password_hash = hash_password(&password)?;  
    // ...  
    Ok(User)  
}
```

##### 2. Implemetation tuỳ chỉnh: 
Khi tạo ra 1 new type wrapper thì việc implement trait để mở rộng chức năng của new type wrapper



#### Nhược điểm 

##### 1. Khó implement dành cho beginner 
##### 2. Tính tương tác: 
Bạn sẽ khó khăn khi convert từ newtype wrapper sang kiểu dữ liệu mà được wrap hoặc ngược lại 
