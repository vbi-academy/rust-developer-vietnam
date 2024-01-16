# Discord Chatbot

## Giới thiệu về HTTP Protocol

## HTTP là gì ?

HTTP (HyperText Transfer Protocol) `giao thức truyền tải siêu văn bản` là một giao thức ứng dụng trong bộ giao thức TCP/IP, được sử dụng để truyền tải dữ liệu giữa máy khách ( client) và máy chủ (server) . HTTP là giao thức cơ bản của World Wide Web (WWW), cho phép người dùng truy cập và tải về các tài nguyên như văn bản, hình ảnh, video, âm thanh

## 

### HTTP Version

Version của HTTP protocol  

### URI

Xác định tài nguyên mà client muốn truy cập 

### HTTP Method

Xác định hành động mà client muốn máy chủ thực hiện : GET, POST, PUT, DELETE

### HTTP Request Headers

Chứa thông tin bổ sung về yêu cầu, ví dụ như loại nội dung (image, json, video, ..), kích thước nội dung 

### HTTP Body

Chứa thông tin sẽ được gửi cho server ( ví dụ username, password khi login ) 

Ví dụ
```
GET /articles/latest HTTP/1.1
Host: www.example.com
Accept: application/json
Connection: Keep-Alive
```
- Phương thức: GET
- URI: /articles/latest
- HTTP version: HTTP/1.1
- Headers: Host ( máy chủ), Accept (loại dữ liệu có thể chấp thuận ) ,  Connection ( mong muốn request cho đến lúc timeout)


Ví dụ khác:

```
POST /users HTTP/1.1
Host: www.example.com
Content-Type: application/json

{
  "email": "johndoe@example.com",
	"password":"123456789"
}
```

## HTTP Response

### HTTP Status Code (Phần mã trạng thái )

XÁc định thành công hay thất bại của yêu cầu từ client ví dụ 

- 200 : Ok
- 404: Không tìm thấy
- 500: Lỗi máy chủ

### HTTP Response Headers

Chứa các thông tin bổ sung về phản hồi, chẳng hạn như loại nội dung, kích thước nội dung,…

### HTTP Response Body

Bao gồm dữ liệu của tài nguyên mà máy khách yêu cầu

```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "message": "Success",
  "data": {
    "name": "Dung",
    "age": 30,
    "occupation": "Blockchain Engineer"
  }
}
```


## Giới thiệu cơ bản về Async  , Await

**`async`** và **`await`** được sử dụng để hỗ trợ lập trình không đồng bộ

`async` Được sử dụng để định nghĩa một hàm hoặc code block làm việc không đồng bộ

**`await`**: Được sử dụng trong một hàm **`async`** để chờ (await) kết quả của một thao tác không đồng bộ

## Các thư viện hỗ trợ HTTP trong Rust

### Reqwest: Higher level HTTP Client

### Hyper : low-level HTTP library

### Axum : web server

### Warp : web server

## Yêu cầu trước khi vào challenge

- Hiểu cơ bản liên quan tới Rust
- Hiểu cơ bản HTTP Request, Response
- Sử dụng framework

## Phần 1: Call HTTP để tương tác với Discord server

- Login
- Get thông tin user
- Logout

Link code: https://github.com/CocDap/Rust-Challenges/tree/main/discord-chatbot/discord-api


## Phần 2: Discord chatbot GM  sử dụng serenity-rs

- Sử dụng framework Serenity-rust tạo chatbot GM

Link code: https://github.com/CocDap/Rust-Challenges/tree/main/discord-chatbot/discord-bot
## Kiến thức học được sau khi xây dựng chatbot bằng Rust

- Error Handling
- Config file
- Cách sử dụng serde, serde_json
- Cách sử dụng reqwest
- Run chatbot

## Tài liệu tham khảo
- Serenity-rs
- https://discord.com/developers/docs/reference


