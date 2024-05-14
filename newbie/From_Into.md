## From và Into Trait trong Rust

### Yêu cầu kiến thức
+ Hiểu cơ bản cách sử dụng và ứng dụng của `Trait`


### Trait `From` và `Into` là gì 

1. `From` và `Into` là 2 trait cung cấp sẵn bởi Rust, nhằm với mục đích chuyển đổi kiểu dữ liệu A sang dữ liệu B và ngược lại một cách an toàn 

2. Có vẻ như các bạn có thể hình dung được nó như cú pháp `as` (chuyển đổi dữ liệu) , nhưng From và Into khác biệt ở chỗ là flexible trong cách chuyển đổi, quản lý chặt chẽ khi chuyển đổi xảy ra lỗi, hỗ trợ complex custom data 

#### From

+ Code:

```rust
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}
```

+ Source: https://doc.rust-lang.org/std/convert/trait.From.html

+ From<T> for U : Muốn chuyển đổi kiểu dữ liệu T sang U dùng method `from`

+ Dựa vào trait `From`, mô tả hành vi cụ thể cho 1 đối tượng cụ thể 

+ Ví dụ : Chuyển độ F sang độ C và ngược lại 

```rust
struct Fahrenheit(f64);
struct Celsius(f64);

// Chuyển độ C sang độ F 
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 1.8 + 32.0)
    }
}

// Chuyển độ F sang độ C 
impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) / 1.8)
    }
}

fn main() {
    let boiling_f = Fahrenheit(212.0);
    let boiling_c = Celsius::from(boiling_f);
    println!("Nhiệt độ sôi của nước: {} độ C", boiling_c.0);

    let freezing_c = Celsius(0.0);
    let freezing_f = Fahrenheit::from(freezing_c);
    println!("Nhiệt độ đóng băng của nước: {} độ F", freezing_f.0);

}
```

4. Một số ví dụ thực tế 
+ `rand` crate : https://github.com/rust-random/rand/blob/master/src/seq/index.rs#L127
+ `base64` crate: https://github.com/marshallpierce/rust-base64/blob/master/src/decode.rs#L78 -> phục vụ cho mục đích trả về lỗi
+ `syn` crate: https://github.com/dtolnay/syn/blob/master/src/lit.rs#L484

+ ...


#### Into

+ Code:

```rust
pub trait Into<T>: Sized {
    // Required method
    fn into(self) -> T;
}
```

+ Source: https://doc.rust-lang.org/std/convert/trait.Into.html

+ `Into<U>`: Nếu đã implement `From<T> for U` thì Rust sẽ tự động implement trait `Into<U> for T`

+ Ví dụ : Dựa vào ví dụ trên, ta đã implement `trait From<T> for U`, nghĩa là implement `trait From<Celsius> for Fahrenheit` và `trait From<Fahrenheit> for Celsius` -> có thể sử dụng `.into()` để chuyển đổi 


```rust
fn main() {
    let boiling_f = Fahrenheit(212.0);
    // Khi dùng into thì cần kèm theo kiểu dữ liệu cần chuyển đổi
    // Chuyển độ F sang C 
    let boiling_c: Celsius = boiling_f.into();
    println!("Nhiệt độ sôi của nước: {} độ C", boiling_c.0);

    // Chuyển độ C sang F 
    let freezing_c = Celsius(0.0);
    let freezing_f: Fahrenheit = freezing_c.into();
    println!("Nhiệt độ đóng băng của nước: {} độ F", freezing_f.0);
}
```

### Kết luận
+ Mối quan hệ: `From<T> for U` và `Into<U> for T` là 2 trait có sẵn được sử dụng thông dụng trong Rust, thường sẽ implement `trait From<T> for U` thì có thể sử dụng `Into<U> for T`

+ Chuyển đổi ko có lỗi : Nghĩa là chuyển đổi kiểu dữ liệu A sang B và ngược lại luôn thành công. Nếu handle được lỗi khi convert , ta có thể sử dụng cặp trait tương tự 
là `TryFrom` và `TryInto`

https://doc.rust-lang.org/std/convert/trait.TryFrom.html


**Bài toán : Convert String sang IPV4Address**

```rust
// Import TryFrom
use std::convert::TryFrom;

// Định nghĩa IpV4Address 
// Ví dụ : 127.0.0.1 -> [127, 0, 0, 1]
#[derive(Debug)]
struct IpV4Address {
    ip: [u8; 4],
}

// Định nghĩa lỗi khi convert string sang IpV4Address
#[derive(Debug)]
enum IpV4AddressError {
    InvalidFormat,
    OutOfRange,
}

// Implement trait TryFrom convert từ String sang IpV4Address
// Đối với trait TryFrom , có thêm associated type Error -> custom lỗi trả về 
impl TryFrom<String> for IpV4Address {
    // Lỗi trả về đang định dạng enum 
    type Error = IpV4AddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // split chuỗi dựa vào dấu . 
        let ip_splited: Vec<&str> = value.split('.').collect();

        // check valid length format 
        if ip_splited.len() != 4 {
            return Err(IpV4AddressError::InvalidFormat);
        }

        let mut ip = [0u8; 4];
        for (i, s) in ip_splited.iter().enumerate() {
            //parse string sang number u8
            match s.parse::<u8>() {
                Ok(number) => ip[i] = number,
                Err(_) => return Err(IpV4AddressError::OutOfRange),
            }
        }

        Ok(IpV4Address { ip })
    }
}

fn main() -> Result<(), IpV4AddressError> {
    let valid_ip_str = "127.0.0.1".to_string();
    let invalid_ip_str = "999.999.999.999".to_string();

    let valid_ip = IpV4Address::try_from(valid_ip_str)?;
    println!("Valid Ip:{:?}", valid_ip);

    let invalid_ip = IpV4Address::try_from(invalid_ip_str)?;
    println!("InValid Ip:{:?}", invalid_ip);
    Ok(())
}
```

Kết quả:

```
Valid Ip:IpV4Address { ip: [127, 0, 0, 1] }
Error: OutOfRange
```

