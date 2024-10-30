## Built-in Trait in Rust
Cung cấp cách sử dụng các trait mà Rust cung cấp sẵn, sử dụng và customized tuỳ vào mục đích của trait đó 

### Yêu cầu kiến thức
+ Hiểu cơ bản cách sử dụng và ứng dụng của `Trait`


### Display 
1. **Code:**
```rust
pub trait Display {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```
2. **Source:** https://doc.rust-lang.org/std/fmt/trait.Display.html


3. **Mục đích:** định dạng sang thông tin user có thể đọc được (human-readable format)

4. **Cách sử dụng**
+ Định dạng dữ liệu cho `{}` (thường sử dụng trong `println!`, `print!`, `format!` ...)
+ Hiển thị thông tin cho logs 

5. **Ví dụ:**
```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write thông tin x và y 
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 1, y: 2 };
    println!("{}", point);
    //(1, 2)
}

```

### Debug 
1. **Code:** 

```rust
pub trait Debug {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

2. **Source:** https://doc.rust-lang.org/std/fmt/trait.Debug.html

3. **Mục đích:** định dạng sang kiểu user có thể đọc được (human-readable format) 

4. **Cách sử dụng:**
+ Định dạng dữ liệu cho `{:?}` (thường sử dụng trong `println!`, `print!`, `format!` ...)
+ Hiển thị thông tin cho logs 


5. **Ví dụ sử dụng `#[derive(Debug)]`**
```rust

// sử dụng derive macro Debug
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    // sử dụng {:?}
    println!("{:?}", point);
    // Point { x: 1, y: 2 }
}
```

6. **Ví dụ `impl Debug for <...>`**

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // hiển thị thông tin cho struct 
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    let point = Point { x: 1, y: 2 };
    println!("{:?}", point);
    //Point { x: 1, y: 2 }
}
```

+ Sử dụng `debug_struct` để hiển thị thông tin `struct Point` trong Rust sang thông tin human-readable
+ Ngoài ra hỗ trợ `debug_map`, `debug_tuple`, `debug_set`, `debug_list` ,... 


### Sự giống nhau và khác nhau giữa Display và Debug 

#### Giống nhau

1. Hiển thị thông tin human-readable 
2. Hỗ trợ logs 
3. Lý do tại sao chúng ta có thể in ra màn hình với các primitives type ( như là u8, u32, ...). Có thể do: 
+ Trait `Display` đã implement cho `Primitives Type` ( Source code: https://doc.rust-lang.org/src/core/fmt/num.rs.html#471-474)
+ Rust đã implement trait `Debug` cho các `Primitives Type` đó ( Source code: https://doc.rust-lang.org/src/core/fmt/num.rs.html#187)

#### Khác nhau

1. **Mô tả mặc định (Default Implementation)**
+ `Display`: thường yêu cầu developer tự implement (ngoài các kiểu dữ liệu mà Rust đã implement sẵn)
+ `Debug`: có thể sử dụng `#[derive(Debug)]` có sẵn của Rust 

2. **Cú pháp** 
+ `Display`: sử dụng `{}`
+ `Debug` : sử dụng `{:?}`


### Default 
1. **Code:**
```rust
pub trait Default: Sized {
    // Required method
    fn default() -> Self;
}
```
2. **Source:** https://doc.rust-lang.org/std/default/trait.Default.html

3. **Mục đích:** Tạo giá trị mặc định cho dữ liệu 

4. **Ví dụ 1**

Chúng ta thấy rằng các kiểu dữ liệu nguyên thuỷ - primitives type (i8, i16, i32, ..), character, boolean, tuple đều có giá trị mặc định
(https://doc.rust-lang.org/src/core/default.rs.html#160)

```rust
let default_value: u32 = Default::default();
println!("Giá trị mặc định của u32 là: {}", default_value);
```

Kết quả:
```
Giá trị mặc định của u32 là: 0
```

5. **Ví dụ 2** 

Trong trường hợp giá trị trả về `Option` (giá trị trả về None)

```rust
fn main() {
    let some_value = Some(42);
    let none_value: Option<u32> = None;

    // Some sẽ unwrap ra giá trị 
    let unwrapped_some = some_value.unwrap_or(Default::default());
    // None sẽ unwrap ra default value 
    let unwrapped_none = none_value.unwrap_or(Default::default());
    // có thể dùng cách khác để unwrap 
    let unwrapped_some_other = some_value.unwrap_or_default();
    let unwrapped_none_other = none_value.unwrap_or_default();

    println!("Unwrapped Some: {}", unwrapped_some); 
    // Unwrapped Some: 42
    println!("Unwrapped None: {}", unwrapped_none); 
    // Unwrapped None: 0

    println!("Unwrapped Some other: {}", unwrapped_some_other); 
    // Unwrapped Some: 42
    println!("Unwrapped None other: {}", unwrapped_none_other); 
    // Unwrapped None: 0
}
```

6. **Ví dụ 3**

Kiểu dữ liệu phức tạp có thể trả về Default giá trị 

Định nghĩa `Counter` ban đầu có giá trị mặc định bằng 0 

+ Định nghĩa `Counter`
```rust
struct Counter {
    count: i32,
}
```

+ Implement trait `Default` cho `Counter`
```rust
impl Default for Counter {
    fn default() -> Self {
        Counter { count: 0 }
    }
}
```

+ Instantiate sử dụng default 

```rust
// Cách 1 
let count = Counter::default();
// Cách 2 
let count: Counter = Default::default();
// Cách 3
let count = Counter {count: 0};
```



7. **Ví dụ 4** 

Sử dụng macro `#[derive(Default)]` thay cho việc `impl Default for <...>`

+ Thực ra nó như nhau, bạn có thể sử dụng 1 trong 2 cách. Đối với `#[derive(Default)]` thì Rust đã implement giùm developer, còn việc sử dụng `impl Default` thì developer tự phải làm 

```rust
#[derive(Default)]
struct Counter {
    count: u32,
}
```
+ Khi ta expand code ta sẽ thấy rằng, `#[derive(Default)]` đã impl trait `Default` cho `struct Counter`

```rust
#[automatically_derived]
impl ::core::default::Default for Counter {
    #[inline]
    fn default() -> Counter {
        Counter {
            count: ::core::default::Default::default(),
        }
    }
}
```

+ Work với generic type ( lưu ý không phải lúc nào cũng sử dụng được, phụ thuộc vào ngữ cảnh của code )

```rust
#[derive(Default)]
struct Counter<T> {
    count: T,
}
```

+ Cách sử dụng
```rust
let count = Counter::<u32>::default();
```














