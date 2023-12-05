## Cách sử dụng Iterator Trait 

### Nó là gì ? 

Nó là 1 interface (trait ) của thư viện chuẩn Rust cung cấp dùng để chạy vòng lặp cho collections như arrays, slices , vector, ...

### Cách sử dụng : 

- impl trait `Iterator`
- có thể thay thế cho việc dùng vòng lặp (for, while,… ) tuỳ chỉnh được nhiều hơn

### Một số ưu điểm:

- Tuỳ chỉnh dễ dàng
- hỗ trợ thêm các method như map , filter, …
- Tường minh

### Ví dụ 1: 

Normal way :

```rust
    for number in 0..10 {
        println!("{}", number);
    }
```

Iterator  way:

```rust
    for number in (0..10).iter() {
        println!("{}", number);
    }
```

### Ví dụ 2: Tuỳ chỉnh với Iterator (Sum of Squares)

Normal way:
```rust
    pub fn sum_of_squares(vals: &[u32]) -> u32 {
        let mut total = 0;
        for val in vals {
            total += val*val;
        }
        total
    }
```
Iterator way:

```rust
    pub fn sum_of_squares(vals: impl Iterator<Item = u32>) -> u32 {
		vals.map(|value| value * value).sum()
    }
```
Ưu điểm khi sử dụng Iterator trong trường hợp này:

+ input đầu vào `vals` có thể là `Vec<u32>`, hoặc `&[u32]`, hoặc kiểu dữ liệu dc trait `Iterator` implement -> dynamic
+ sử dụng method `map` và `sum` -> tường minh 




