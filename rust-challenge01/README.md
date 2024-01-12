## Giới thiệu bản thân

- Hồ Đình Dụng - Blockchain Engineer - Lecturer in VBI Academy

## Yêu cầu trước khi vào Challenge

- Cài đặt Rust và môi trường
- Biết cơ bản về Rust
- Sẵn sàng thử thách đầu tiên

## Mục đích

Làm quen với Rust để implement các ứng dụng cụ thể - JSON Parser ( phân tích cú pháp của dữ liệu đó có phải là loại JSON hay không )

## JSON

- Javascript Objection Notion là dạng dữ liệu nhẹ , linh hoạt , dễ sử dụng, tạo thành từ cặp khoá -giá trị (key-value)
- Khoá là 1 chuỗi (string)
- Giá trị có thể là 1 chuỗi (string) , số (number) , mảng(array) hoặc đối tượng (object)

### Ví dụ về json

```json
{
  "title": "Rust Challenge",
  "year": 2023,
  "live": true,
  "organizers": ["vbi", "techfest"],
  "presenter": {
    "name": "Dung",
    "age": 27,
    "occupation": "Blockchain Engineer"
  }
}
```
### Ứng dụng thực tế của JSON

- Data exchange: API

`https://catfact.ninja/fact`

`{"fact":"A healthy cat has a temperature between 38 and 39 degrees Celcius.","length":66}`

- Configuration Files: https://cloud.google.com/appengine/docs/admin-api/creating-config-files

- Data Storage: JSON database (MongoDB sử dụng phiên bản BSON là phiên bản binary JSON hoặc một số DB như CosmosDB, …)

JSON: `{ "name": "John", "age": 30 }`

### Một số JSON parsers có sẵn ở một số ngôn ngữ lập trình

Javascript: `JSON.parse()` function

Python: `json` module

Rust : `serde_json` crate

…

## Bài toán

Làm thế nào để parse đầu vào `string` sang JSON format 

### Phân tích đầu vào/ đầu ra:

- Đầu vào : Chuỗi ký tự
- Đầu ra: JSON format

### Bạn sẽ học được những gì trong buổi học hôm nay:

- Sử dụng if let Some
- Error Handling
- Trait Iterator
- Viết unit test

### Cung cấp một số kiến thức cơ bản trước khi implement JSON Parser

- Trait - Generic type
- If let Some
- Phân biệt cách sử dụng iter() và peek()

### Cách giải quyết

Vì cấu trúc JSON có chuẩn , dựa vào chuẩn ta có các quan sát như sau : 

- Parse string : bắt đầu bằng kí tự `"` và kết thúc bằng kí tự `"`
- Parse number : kiểm tra có phải là number hay không
- Parse boolean: kiểm tra có phải true hay false
- Parse array: bắt đầu bằng kí tự `[`
- Parse object: bắt đầu bằng kí tự `}`


### Testing

+ Trường hợp 1: JSON rỗng
```json
{}
```

+ Trường hợp 2: JSON có 1 key và 1 value 
```json
{"key":"value"}
```

+ Trường hợp 3: JSON có nhiều key và nhiều value (string, number, boolean)
```json
{
  "key1": true,
  "key2": false,
  "key4": "value",
  "key5": 101
}
```

+ Trường hợp 4: Nested JSON
```json
{
  "key": "value",
  "key1": 101,
  "key2": {},
  "key3": []
}
```
+ Trường hợp 5: 

```json
{
  "title": "Rust",
  "year": 2023,
  "live": true,
  "organizers": ["vbi", "techfest"],
  "presenter": {
    "name": "Dung",
    "age": 27,
    "occupation": "Engineer"
  }
}
```