# Xây dựng Web-Service đơn giản với Rust và Axum

Dự án này là một ví dụ đơn giản về cách tạo một web service sử dụng Rust và thư viện Axum. Axum là một web framework cho phép tạo các ứng dụng HTTP một cách nhanh chóng và dễ dàng với Rust.

# Kết quả mong muốn đạt được sau bài học

- Hiểu cơ bản về ngôn ngữ lập trình Rust
- Biết cách hoạt động của Web-Service: Request, Response, Basic API, ...
- Hiểu cách kết nối cơ bản với cơ sở dữ liệu (trong bài này là PostgreSQL)
- Biết cách sử dụng cơ bản SeaORM để kết nối và thực hiện các câu truy vấn từ **Rust** và database **PostgreSQL**

# Hướng dẫn
## Cài đặt các thư viện (Crates) cần thiết

Chỉnh sửa file `Cargo.toml` của dự án và thêm các dependency sau:

``` rust
[dependencies]
axum = "0.7.5"
bcrypt = "0.15.1"
chrono = { version = "0.4.38", features = ["serde"] }
dotenv = "0.15.0"
serde_json = "1.0.117"
axum-macros = "0.4.1"
jsonwebtoken = "9.3.0"
serde = { version = "1.0.202", features = ["derive"] }
tokio = { version = "1.37.0", features = ["full"] }
```


> [!NOTE]
> **Lưu ý:** Các phiên bản có thể thay đổi tùy thuộc vào thời điểm bạn thực hiện.
### **Giải thích:**

##### Web Service và API:

- **axum:** Framework web để xây dựng API server.
- **axum-macros:** Macro hỗ trợ viết mã gọn hơn cho Axum, hỗ trợ bắt lỗi cho router chạy ổn định hơn.
- **jsonwebtoken:** Thư viện xác thực và ủy quyền người dùng trong ứng dụng web, bao gồm tạo và phân tích JWT token để xác thực.

##### **Xử lý dữ liệu:**

- **serde:** Framework serialization/deserialization cho Rust.
    - Thêm `features = ["derive"]` để tự động generate code cho việc **Serialization** và **Deserialization**.
- **serde_json:** Thư viện serialization/deserialization cho JSON.
- **chrono:** Thư viện xử lý ngày giờ và thời gian.
 
##### **Bảo mật:**

- **bcrypt:** Thư viện bcrypt sử dụng để băm mật khẩu trước khi lưu vào cơ sở dữ liệu để bảo mật cho người dùng. bcrypt cũng có thể giải mã ngược để xác thực đăng nhập.

##### **Môi trường và Cấu hình:**

- **dotenv :** Tải biến môi trường từ file .env.

**Runtime Asynchronous:**

- **tokio:** Runtime cho phép viết mã bất đồng bộ.
	- Thêm `features = ["full"]` để bật **tất cả** các tính năng mà Tokio cung cấp.

## Kết nối SeaORM 

### 1. Khái niệm cơ bản

SeaORM là một **ORM** (Object-Relational Mapper) cho ngôn ngữ lập trình Rust, được thiết kế để đơn giản hóa việc tương tác với các cơ sở dữ liệu quan hệ. Với SeaORM, bạn có thể tập trung vào logic nghiệp vụ của ứng dụng mà không cần phải viết quá nhiều mã SQL boilerplate.

***Các khái niệm cơ bản:***

- **Entity:** Cấu trúc Rust đại diện cho một bảng trong cơ sở dữ liệu.
- **Model:** Một instance cụ thể của Entity, tương đương với một dòng dữ liệu trong bảng.
- **Schema:** Mô tả cấu trúc toàn bộ cơ sở dữ liệu, bao gồm các Entity và mối quan hệ giữa chúng.
- **Query Builder:** Xây dựng các truy vấn SQL an toàn và dễ đọc bằng code Rust.
- **Migration:** Quản lý các thay đổi schema theo thời gian.

***Ưu điểm của SeaORM:***

- **Type-safe:** Kiểm tra kiểu dữ liệu ngay khi biên dịch.
- **Asynchronous:** Hỗ trợ async/await, cho phép xử lý nhiều request đồng thời.
- **Dễ sử dụng:** Cú pháp đơn giản, dễ học.
- **Hỗ trợ nhiều cơ sở dữ liệu:** PostgreSQL, MySQL, SQLite,..

***Nhược điểm:***

* **Hiệu năng:**  Trong một số trường hợp, ORM có thể ảnh hưởng đến hiệu năng so với viết SQL trực tiếp. Tuy nhiên, SeaORM được tối ưu hóa và thường không phải là vấn đề trong hầu hết ứng dụng.
* **Khả năng tùy biến:**  ORM có thể giới hạn khả năng tùy biến truy vấn SQL.


### 2. Hướng Dẫn Cài Đặt
#### Kết nối Sea-ORM

##### ***2.1.1 Thư viện (Crates) cần***:
  
Bổ sung **sea-orm** và các dependencies liên quan vào file **Cargo.toml**:

``` rust
sea-orm = { version = "0.12.15", features = ["sqlx-postgres","runtime-tokiorustls","macros",] }
```

 **Giải thích:**

**sea-orm:** Thư viện ORM.
- **features = ["postgres", "runtime-tokio-rustls", "chrono"]**:
    - postgres: Hỗ trợ kết nối PostgreSQL.
    - runtime-tokio-rustls: Tương tự như trên.
    - chrono: Hỗ trợ kiểu dữ liệu chrono cho ngày giờ.

------------------------------------------------------------------------

##### ***2.1.2 CLI tạo migrate  và entity***:
**Giới Thiệu**

Bổ sung Crate vào `Cargo.toml`
```rust
[workspace]
embers = [".", "entity", "migration"]


[dependencies]
migration = { path = "migration" }
entity = { path = "entity" }
```

###### **Giải thích:**

*- Trong SeaORM, một **Entity** đại diện cho một bảng trong cơ sở dữ liệu. Entity định nghĩa cấu trúc của bảng, bao gồm tên bảng, tên cột, kiểu dữ liệu và các ràng buộc.
- **Migration** là quá trình cập nhật lược đồ cơ sở dữ liệu dựa trên những thay đổi trong định nghĩa Entity

***1. CLI để tạo migration:

``` bash
sea-orm-cli migrate init
sea-orm-cli migrate generate <tên migration>
sea-orm-cli migrate status
sea-orm-cli migrate refresh
```

####### **Giải thích:**
* **sea-orm-cli migrate init :** Khởi tạo thư mục migration cho dự án SeaORM.
* **sea-orm-cli migrate generate <tên migration> :** Tạo một file migration mới với tên <tên migration>.
* **sea-orm-cli migrate status :** Hiển thị trạng thái của các migration trong dự án.
* **sea-orm-cli migrate refresh :** Reset database về trạng thái ban đầu (xóa tất cả bảng) và chạy lại tất cả các migration

![[run migrate.png]]
> [!WARNING]
> **NOTE** : khi chạy refresh sẽ xóa toàn bộ dữ liệu đang có trong cơ sở dữ liệu

> [!NOTE]
> Nếu gặp lỗi liên quan đến cấp quyền (GRANT) bạn có thể vào bài viết này để tham khảo 
> [link](https://github.com/PhucLam202/Denied-Schema-Public/tree/main)

```base 
src
├── migration
│   ├── main.rs
│   └── m20240526_063346_create_user_table.rs
|   └── lib.rs
```

**Trong đó **
- **main.rs** : là nơi chạy lệnh CLI thực thi file migrate.rs 
- **lib.rs** : là nơi khai báo cấu trúc các file migrate và thứ tự thực thi các file migrate
- **m20240526_063346_create_user_table.rs** : là file ánh xạ dữ liệu với CSDL.


> [!NOTE]
> Dưới đây là file mẫu của các thư mục khi bạn chạy init lần đầu

`main.rs`
``` rust
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
cli::run_cli(migration::Migrator).await;
  }
```

`lib.rs`
```rust
pub use sea_orm_migration::prelude::*;
mod m20240526_063346_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20240526_063346_create_user_table::Migration),
		]
	}
}
```

`m20240526_063346_create_user_table.rs`
```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}

```


> [!IMPORTANT]
> Hãy xóa hoặc comment **todo!()**; để code có thể thực thi tránh bị lỗi 


***CLI để tạo entity

``` bash
	sea-orm-cli generate entity -o your entity file output directory
```

	Example : sea-orm-cli generate entity -o entity/src

 ###### **Giải thích:**
 
 - **sea-orm-cli generate entity** : Khởi tạo ra 1 thư mục tên entity chứ các file ánh xá trực tiếp dữ liệu có trong CSDL 
	 - -o/ --output-dir: thư mục đầu ra file thực thể (mặc định: thư mục hiện tại)

``` rust 
src
├── entity
│   ├── src
│   └──── mod.rs
|	└──── prelude.rs
|	└──── your_table_name_in_DB 
```

Trong đó 
	- **mod.rs** : khai báo các file có trong thư mục src để các module khác có thể hiểu
- **prelude.rs** : khai báo đối tượng để tái xuất thực thể `Entity` từ module `users` của module cha, đổi tên thành `Users`, và cho phép người dùng bên ngoài module hiện tại truy cập vào nó.
- **your_table_name_in_DB .rs** : Là thư mục hiển các columns có trong CSDL

![[run entity.png]]

`your_table_name_in_DB` 
```rust
//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
#[sea_orm(primary_key)]
	pub id: i32,
	pub username: Option<String>,
	pub email: Option<String>,
	pub phone: Option<i32>,
	pub created_at: Option<DateTime>,
	pub updated_at: Option<DateTime>,
	pub password: Option<String>,
	pub sofl_delete: Option<bool>,
	pub is_active: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

> [!NOTE]
Có 2 cách tiếp cận khi sử dụng sea-orm là `Schema First` và `Entity First`
Trong hướng dẫn này thì mình tiếp cận theo hướng `Schema First` (chạy migration để tạo ra các table trong database rồi dựa vào đó để generate ra entity)

## Kết nối Server 
### 1. Khái niệm cơ bản 

- **Server** là một chương trình phần mềm hoặc một máy tính hoạt động như một trung tâm cung cấp dịch vụ cho các chương trình khác, được gọi là **client**. Trong mô hình client-server, client sẽ gửi request (yêu cầu) đến server, và server sẽ xử lý yêu cầu đó và gửi response (phản hồi) trở lại cho client.

- **Web server** là một loại server chuyên xử lý các request liên quan đến web, ví dụ như request cho các trang web, hình ảnh, video, và các tài nguyên web khác. Web server sử dụng giao thức HTTP (Hypertext Transfer Protocol) để giao tiếp với client.

- **Axum** là một web framework cho Rust, được xây dựng dựa trên Tokio, một runtime asynchronous cho Rust. Axum cung cấp các công cụ và abstraction để xây dựng web server và API một cách dễ dàng, hiệu quả và an toàn.
- 
**Ưu điểm:**

- **Siêu nhanh, siêu mượt:** Xử lý lượng request khổng lồ một cách nhẹ nhàng.
- **An toàn bậc nhất:** Ngăn chặn lỗi bộ nhớ, đảm bảo server ổn định.
- **Dễ dàng mở rộng:** Xử lý song song, "cân" mọi lượng truy cập.
- **Cộng đồng chất:** Nhiệt tình, hỗ trợ, giúp bạn giải quyết mọi vấn đề.

**Nhược điểm:**

- **Khó "nhai" ban đầu:** Rust đòi hỏi thời gian để làm quen, đặc biệt là ownership và borrowing.
- **Hệ sinh thái còn non trẻ:** Ít thư viện hơn so với các ngôn ngữ lâu đời.
- **Tài liệu hạn chế:** Khó khăn cho người mới bắt đầu.

### 2. Hướng dẫn cài đặt 
#### Kết nối rust với CSDL

**phần tiếp theo ta sẽ tiến hành kết nối cơ sở dữ liệu là postgres với rust**

 1. ta cần tạo 1 file tên .env nơi quản lý các chuổi kết nối cơ sỡ dữ liệu 
 ``` rust
DATABASE_URL=protocol://username:password@host/database
```

> [!NOTE]
> thay đổi username, password và database thành thông tin đúng với CSDL của bạn

2. tiếng hành kết nối với CSDL

> [!NOTE]
  Do tôi thực hiện code theo mô hình MVC cơ bản nên sẽ chia file kết nối CSDL thành một thư mục riêng `Server`

```rust
use std::env;
use dotenv::dotenv;

pub async fn conn_db()->String{
	dotenv().ok();
	env::var("DATABASE_URL").expect("Data Must Be Set")
	}
```

- Trong hàm trên tôi đã gọi đến chuổi kết nối đã được khai báo trong file .env để kết nối CSDL 

3. Tiến hành kết nối CSDL thông qua hàm main

``` rust
#[tokio::main]
async fn main() {

	let postgres_url = conn_db().await;
	let db = match Database::connect(&postgres_url).await {
			Ok(db) => db,
			Err(err) => {
		println!("Failed to connect to PostgreSQL: {}", err);
		std::process::exit(1);
		}
	};

	let db = Arc::new(db);
	let user_routes = user_router(Extension(db.clone()));
	let app = Router::new()
			.merge(user_routes)
			.layer(Extension(db));
			
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
```

-  **Kết nối CSDL:**
    - **conn_db().await**: Giả sử hàm conn_db() (không được định nghĩa trong đoạn code bạn cung cấp) có nhiệm vụ đọc URL kết nối PostgreSQL từ file cấu hình hoặc môi trường và trả về một String chứa URL đó.

    - **Database::connect(&postgres_url).await**: Thực hiện kết nối đến CSDL PostgreSQL sử dụng URL đã đọc được. Kết quả trả về là Result<DatabaseConnection, DbErr>, trong đó:
	    - ***Ok(db) :*** Kết nối thành công, db là đối tượng DatabaseConnection đại diện cho kết nối.    
        - ***Err(err) :*** Kết nối thất bại, err chứa thông tin lỗi.
    - **Xử lý kết quả kết nối**:
        - => *Nếu thành công*, gán db cho biến db.
        - => *Nếu thất bại*, in thông báo lỗi ra console và kết thúc chương trình với code 1 (thường biểu thị lỗi).

-  **Chia sẻ kết nối CSDL:**
    
    - **Arc::new(db):** Tạo một Arc <sup>Atomically Reference Counted</sup> pointer đến db. Điều này cho phép chia sẻ kết nối CSDL một cách an toàn giữa các phần khác nhau của ứng dụng, đồng thời đảm bảo kết nối vẫn tồn tại trong suốt thời gian chạy của ứng dụng.
    
    - Sử dụng **Extension(db.clone())** để truyền bản sao Arc của kết nối CSDL vào router user_router và router chính của ứng dụng. Điều này cho phép các handler trong các router này sử dụng kết nối CSDL.

-  **Khởi động Server:**
    
    - Tạo listener lắng nghe kết nối đến trên cổng `3000 `(tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap()).
    
    - Khởi chạy web server Axum sử dụng listener đã tạo và router chính đã được gắn kết (axum::serve(listener, app).await.unwrap()).

**Để chạy dự án dùng câu lệnh **
 ``` rust
 cargo run 
```


## Khỏi Tạo cấu trúc dự án 

### Khái niệm cơ bản 
#### Giới thiệu cấu trúc MVC

**MVC**, viết tắt của **Model-View-Controller**, là một cách tổ chức code giúp ứng dụng web rõ ràng, dễ quản lý hơn.

**Tưởng tượng:**

- **Model:** Như "kho dữ liệu", lưu trữ và quản lý thông tin (ví dụ: bài viết, người dùng).
- **View:** "Mặt tiền" của ứng dụng, hiển thị thông tin cho người dùng (ví dụ: trang web, giao diện app).
- **Controller:** "Bộ não", nhận lệnh từ người dùng, giao tiếp với Model và View để xử lý yêu cầu.

**Lợi ích:**
- Code dễ hiểu, dễ bảo trì.
- Dễ dàng mở rộng và tái sử dụng code.
- Kiểm tra lỗi dễ dàng hơn.

### Hướng Dẫn

#### Cấu trúc dự án 
##### Cấu trúc chung dự án 

```
src
├── controllers
│   ├── mod.rs
│   └── users_controller.rs
├── helpers
│   ├── api_error.rs
│   └── mod.rs
├── main.rs
├── middleware
│   ├── auth.rs
│   └── mod.rs
├── models
│   ├── mod.rs
│   └── users_model.rs
├── routers
│   ├── mod.rs
│   └── user_router.rs
└── server
    ├── mod.rs
    └── postgres_server.rs

```

**Giải thích:**

- **src**: Chứa toàn bộ mã nguồn Rust của dự án.

- **controllers**:
    - Chứa các controller, chịu trách nhiệm xử lý logic nghiệp vụ và yêu cầu từ client.
    - **`mod.rs`**: File module chính của thư mục `controllers`, dùng để khai báo và export các module con bên trong.
    - **`users_controller.rs`**:  Chứa controller xử lý logic liên quan đến `users`.

- **helpers**:
    - Chứa các hàm tiện ích, các đoạn code dùng chung cho nhiều phần khác nhau của dự án.
    - **`api_error.rs`**:  Có thể chứa định nghĩa struct `ApiError` để xử lý lỗi API.
    - **`mod.rs`**:  File module chính của thư mục `helpers`.

- **main.rs**: Điểm vào chính của ứng dụng, nơi khởi tạo server và các thành phần khác.

- **middleware**:
    - Chứa các middleware, được thực thi trước hoặc sau các handler function để xử lý các tác vụ như xác thực (authentication), logging, ...
    - **`auth.rs`**: Chứa middleware xử lý xác thực người dùng.
    - **`mod.rs`**: File module chính của thư mục `middleware`.

- **models**:
    - Chứa các model, đại diện cho cấu trúc dữ liệu của ứng dụng (ví dụ: database models).
    - **`mod.rs`**: File module chính của thư mục `models`.
    - **`users_model.rs`**:  Chứa model `User` tương tác với database.

- **routers**:
    - Định nghĩa các routes (đường dẫn) của ứng dụng và ánh xạ chúng với các controller tương ứng.
    - **`mod.rs`**:  File module chính của thư mục `routers`.
    - **`user_router.rs`**:  Định nghĩa các route liên quan đến `users`.

- **server**:
    -  Chứa logic khởi tạo và cấu hình server, có thể bao gồm kết nối database.
    -  **`mod.rs`**: File module chính của thư mục `server`.
    -  **`postgres_server.rs`**: Có thể chứa code kết nối đến database PostgreSQL.

------------------------------------------------------------------------
##### **Error**

```rust
use axum::{
		http::{header, StatusCode},
		response::IntoResponse,
		Json,
	};
use serde_json::json;


pub struct APIerror {
	pub message: String,
	pub status_code: StatusCode,
}

  
impl IntoResponse for APIerror {
	fn into_response(self) -> axum::response::Response {
		let status_code = self.status_code;
		   (status_code,[(header::CONTENT_TYPE,"application/json")], Json(json!({
			"StatusCode": self.status_code.as_u16(),
			"Message": self.message }))
		).into_response()
	}
}
```

- **Định nghĩa cấu trúc `APIerror`**
    
    - Bao gồm thông điệp lỗi và mã trạng thái HTTP.

- **Triển khai `IntoResponse`**
    
    - Chuyển đổi `APIerror` thành phản hồi HTTP với:
        - Mã trạng thái từ `APIerror`.
        - Tiêu đề `Content-Type` là `application/json`.
        - Nội dung JSON chứa mã trạng thái và thông điệp lỗi.

>[!NOTE]
>Do APIerror sẽ dùng khá nhiều nên tôi sẽ đưa lên đầu tiên 

------------------------------------------------------------------------
##### **Controller**

=> ***User.rs***
**1**. ***Hàm để kết nối đến CSDL***

```rust
async fn db_connection() -> DatabaseConnection {
	let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let db = Database::connect(db_url).await.unwrap();
	db
}
```
Trong đoạn code trên tôi khai báo hàm `db_connection` để kết nối với CSDL và có thể sữ dụng lại nhiều lần trong các hàm khác  

- **db_url** :  Đọc giá trị của biến môi trường có tên là "DATABASE_URL". Biến môi trường này thường chứa URL kết nối đến cơ sở dữ liệu.
- **db** : Đây là phần chính thực hiện kết nối đến cơ sở dữ liệu
	-  **Database::connect(db_url)**: Gọi hàm **connect()** của kiểu dữ liệu Database. Hàm này nhận `URL database` từ bước trước làm tham số đầu vào.

**2**. ***Hàm tạo mới người dùng***

```rust
pub async fn create_user(Json(user_data): Json<CreateRespore>) -> Result<Json<APIResponse>, APIerror> {

	dotenv().ok();
	let conn = db_connection().await;
	let hashpassword = hash(&user_data.password, DEFAULT_COST).map_err(|err| APIerror {
		message: err.to_string(),
		status_code: StatusCode::INTERNAL_SERVER_ERROR,
	});
	let user = users::ActiveModel {
		username: Set(Some(user_data.email.to_owned())),
		email: Set(Some(user_data.email.to_owned())),
		password: Set(Some(hashpassword?.to_owned())),
		updated_at: Set(Some(Utc::now().naive_local())),
		created_at: Set(Some(Utc::now().naive_local())),
		sofl_delete: Set(Some(false)),
		is_active: Set(Some(true)),
		..Default::default()
	};

	let _ =user.insert(&conn)
		.await
		.map_err(|err| APIerror {
			message: err.to_string(),
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
		})
		.map(|_model| ());
	Ok(Json(APIResponse {
		message: "Create Success".to_string(),
	}))
}
```

 - **Khởi tạo**

	 - Hàm `create_user` nhận dữ liệu người dùng từ HTTP request (định dạng JSON) thông qua tham số `user_data`.
	-  `Extension(db)`: Lấy tham số `db` kiểu `Arc<DatabaseConnection>` từ extension.
	
 -  **Mã hóa mật khẩu**:
 
	 - Hàm `hashpassword` sẽ mã hóa mật khẩu người dùng bằng hàm `hash()` với tham số `DEFAULT_COST` để tăng cường bảo mật. Quá trình mã hóa cũng bao gồm xử lý lỗi, trả về mã lỗi 500 nếu gặp sự cố.
	- Xử lý lỗi nếu có bằng cách trả về Err(APIerror) với mã lỗi 500 (Internal Server Error).

 -  **Tạo ActiveModel**:
 
	-  Sử dụng `ActiveModel` từ SeaORM để tạo một bản ghi người dùng mới,  gán các giá trị từ `user_data` và mật khẩu đã mã hóa vào các trường tương ứng. 
	 - **Default::default()** : Đây là cách gọi hàm default() của trait Default. Trait Default cho phép bạn định nghĩa giá trị mặc định cho một kiểu dữ liệu.
 
 - **Lưu vào CSDL**:
 
    -  `user.insert(&conn)` chèn bản ghi người dùng mới vào cơ sở dữ liệu, sử dụng kết nối `conn` đã thiết lập. 
    -  Kết quả trả về từ thao tác `insert` được xử lý để kiểm tra lỗi và bỏ qua kết quả (vì ta chỉ quan tâm đến việc chèn thành công hay không).

- **Trả về kết quả**

    -   Nếu mọi bước diễn ra suôn sẻ, hàm trả về mã HTTP 200 (OK) cùng thông báo "Create Success" trong JSON response.

 **3**. ***Hàm login của người dùng***. 
 ```rust
pub async fn login_user(Extension(db): Extension<Arc<DatabaseConnection>>,
Json(user_data): Json<LoginReq>,) -> Result<Json<LoginResponse>, APIerror> {
	let conn_db = db.as_ref();
	match users::Entity::find()
		.filter(users::Column::Email.eq(user_data.email))
		.one(conn_db)
		.await
	{
		  Ok(Some(user)) => {
		    if let Some(ref hashed_password) = user.password {
			  match verify(&user_data.password, &hashed_password) {
				Ok(matches) if matches => {
				   match create_jwt(
					&user.id.to_string(),
					user.username.as_ref().expect("REASON"),
				) {
					Ok(token) => {
						let token = LoginResponse {
						token,
						message: "Login Success".to_string(),
					   };
						Ok(Json(token))
				  }
				Err(_) => Err(APIerror {
					message: "Login False ".to_string(),
					status_code: StatusCode::INTERNAL_SERVER_ERROR,
					}),
			      }
				}
				Ok(_) => Err(APIerror {
					message: "Wrong password".to_string(),
					status_code: StatusCode::NOT_FOUND,
				   }),
				Err(_) => Err(APIerror {
					message: "False to verity password".to_string(),
					status_code: StatusCode::CONFLICT,
				}),	
			  }
		  } else {
			Err(APIerror {
				message: "Wrong password".to_owned(),
				status_code: StatusCode::NOT_FOUND,
			})
		  }
		}
	Ok(None) => Err(APIerror {
		message: "user not found".to_string(),
		status_code: StatusCode::NOT_FOUND,
		}),
	Err(_) => Err(APIerror {
		message: "Database error".to_string(),
		status_code: StatusCode::INTERNAL_SERVER_ERROR,
		}),
	}
}
```

- **Khởi tạo**

	 - Hàm `login_user` nhận dữ liệu người dùng từ HTTP request (định dạng JSON) thông qua tham số `user_data`.
	- `Extension(db)`: Lấy tham số `db` kiểu `Arc<DatabaseConnection>` từ extension.

- **Tìm kiếm người dùng trong database:**

    -  `let conn_db = db.as_ref();`:  Lấy reference đến kết nối database.
	- Sử dụng SeaORM để tìm kiếm người dùng theo email:
    - `users::Entity::find()`:  Bắt đầu query.
    - `.filter(users::Column::Email.eq(user_data.email))`: Thêm điều kiện lọc theo email từ `user_data`.
    - `.one(conn_db)`: Lấy một kết quả duy nhất.

- **Kiểm tra mật khẩu:**

	- Kiểm tra xem người dùng có mật khẩu đã hash trong database hay không.
	- `if let Some(ref hashed_password) = user.password`:  Lấy giá trị `hashed_password` nếu có.

- **So sánh mật khẩu đã hash:**

	-  Sử dụng hàm `verify()` (bạn cần cung cấp định nghĩa, có thể từ library bcrypt) để so sánh mật khẩu từ `user_data` với mật khẩu đã hash trong database.
	-  Xử lý kết quả từ `verify()`:
	    - `Ok(matches) if matches`: Mật khẩu khớp.
	    - `Ok(_) `: Mật khẩu không khớp.
	    - `Err(_) `:  Gặp lỗi khi xác minh.

- **Tạo JWT token:**

	- Gọi hàm `create_jwt()` (bạn cần cung cấp định nghĩa) để tạo JWT token, truyền vào id và username của người dùng.
	- Xử lý kết quả từ `create_jwt()`:
	    -  `Ok(token)`: Tạo token thành công.
	    -  `Err(_) `:  Gặp lỗi khi tạo token.

- **Xử lý kết quả đăng nhập:**

	-  Trả về  `Ok(Json(LoginResponse { ... }))` chứa token và thông báo nếu đăng nhập thành công. 
	-  Trả về `Err(APIerror { ... })`  với message và status code tương ứng nếu gặp lỗi.


![[LoginUser.png]]


**4**. ***Hàm Update người dùng***
```rust
pub async fn update_user(
	Extension(db): Extension<Arc<DatabaseConnection>>,
	Path(id): Path<i32>,
	Json(user_data): Json<EditUserReq>,
	) -> Result<Json<APIResponse>, APIerror> {
	let conn_db = db.as_ref();
	let hashpassword = hash(&user_data.new_password, DEFAULT_COST)
		.map_err(|err| APIerror {
		  message: err.to_string(),
		  status_code: StatusCode::INTERNAL_SERVER_ERROR,
	});
	
	let mut user: entity::users::ActiveModel = users::Entity::find()
		.filter(users::Column::Id.eq(id))
		.one(conn_db)
		.await
		.map_err(|err| APIerror {
			message: err.to_string(),
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
		})?
		.ok_or_else(|| APIerror {
			message: "User not found".to_string(),
			status_code: StatusCode::NOT_FOUND,
		})?.into();

	user.email = Set(Some(user_data.new_email));
	user.username = Set(Some(user_data.new_username));
	user.phone = Set(Some(user_data.new_phone));
	user.updated_at = Set(Some(Utc::now().naive_local()));
	user.password = Set(Some(hashpassword?));
  
	user.update(conn_db).await.map_err(|err| APIerror {
		message: err.to_string(),
		status_code: StatusCode::INTERNAL_SERVER_ERROR,
	})?;

	Ok(Json(APIResponse {
		message: "Change Success".to_string(),
	}))
}
```

- **Khởi tạo**

	 - Hàm `update_user` nhận dữ liệu người dùng từ HTTP request (định dạng JSON) thông qua tham số `user_data`.
	-  `Extension(db)`: Lấy tham số `db` kiểu `Arc<DatabaseConnection>` từ extension.
	- `Path(id)`: Lấy `id` người dùng từ đường dẫn URL.

- **Tạo hàm hash mật khẩu**
	- Hàm `hashpassword` sẽ mã hóa mật khẩu người dùng bằng hàm `hash()` với tham số `DEFAULT_COST` để tăng cường bảo mật. Quá trình mã hóa cũng bao gồm xử lý lỗi, trả về mã lỗi 500 nếu gặp sự cố.
 
- **Tìm kiếm người dùng theo id**

	- `let mut user: entity::users::ActiveModel = users::Entity::find() ... .into();`: Tìm kiếm người dùng theo `id` và chuyển đổi kết quả tìm kiếm thành mô hình `ActiveModel`.
	- Nếu người dùng không tồn tại, trả về lỗi `User not found`.
	
- **Cập nhật thông tin người dùng**:

	- Gán các giá trị mới từ `user_data` vào các trường tương ứng của `user`.
	- `user.updated_at = Set(Some(Utc::now().naive_local()));`: Cập nhật thời gian chỉnh sửa.
	- `user.password = Set(Some(hashpassword.to_owned()));`: Cập nhật mật khẩu đã được hash.
- - **Lưu các thay đổi vào cơ sở dữ liệu**:
    
    - `user.update(conn_db).await.map_err(|err| APIerror { ... })?;`: Cập nhật thông tin người dùng trong cơ sở dữ liệu và xử lý lỗi nếu có.
    - 
- **Trả về kết quả**:
    
    - `Ok(Json(APIResponse { message: "Change Success".to_string(), }))`: Trả về kết quả thành công dưới dạng JSON với thông báo "Change Success".


**5**. **Hàm delete_user**
```rust
pub async fn delete_user(
	Extension(db): Extension<Arc<DatabaseConnection>>,
	Path(id): Path<i32>,
) -> Result<Json<APIResponse>, APIerror> {
	let conn_db = db.as_ref();
	let mut user: entity::users::ActiveModel = users::Entity::find()
			.filter(users::Column::Id.eq(id))
			.one(conn_db)
			.await
			.map_err(|err| APIerror {
				message: err.to_string(),
				status_code: StatusCode::INTERNAL_SERVER_ERROR,
		})?
			.ok_or_else(|| APIerror {
				message: "User doesn't exists".to_string(),
				status_code: StatusCode::NOT_FOUND,
		})?.into();
		
	user.is_active = Set(Some(false));
	user.sofl_delete = Set(Some(true));

	user.update(conn_db).await.map_err(|err| APIerror {
		message: err.to_string(),
		status_code: StatusCode::INTERNAL_SERVER_ERROR,
	})?;
	
	Ok(Json(APIResponse {
		message: "Delete Success".to_string(),
	}))
}

```
- **Khởi tạo**

	 - Hàm `delete_user` nhận dữ liệu người dùng từ HTTP request (định dạng JSON) thông qua tham số `user_data`.
	- `Extension(db)`: Lấy tham số `db` kiểu `Arc<DatabaseConnection>` từ extension.
	-  `Path(id)`: Lấy `id` người dùng từ đường dẫn URL.
	
-  **Tìm kiếm người dùng theo id**

	- `let mut user: entity::users::ActiveModel = users::Entity::find() ... .into();`: Tìm kiếm người dùng theo `id` và chuyển đổi kết quả tìm kiếm thành mô hình `ActiveModel`.
	- Nếu người dùng không tồn tại, trả về lỗi `User not found`.
	
- **Cập nhật trạng thái người dùng**:

	- `user.is_active = Set(Some(false));`: Đặt trạng thái `is_active` của người dùng thành `false`.
	- `user.soft_delete = Set(Some(true));`: Đặt cờ `soft_delete` của người dùng thành `true`
	
- **Lưu các thay đổi vào cơ sở dữ liệu**:
    
    - `user.update(conn_db).await.map_err(|err| APIerror`: Cập nhật thông tin người dùng trong cơ sở dữ liệu và xử lý lỗi nếu có.
    
- **Trả về kết quả**:
    
    - `Ok(Json(APIResponse { message: "Delete Success".to_string(), }))`: Trả về kết quả thành công dưới dạng JSON với thông báo "Delete Success".

![[DeleteUser.png]]

**6**. **Hàm get_all_user**

``` rust
pub async fn get_all_user(Extension(db): Extension<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<GetAllUser>>, StatusCode> {

	let conn_db = db.as_ref();
	match users::Entity::find().all(conn_db).await {
		Ok(users) => {
			let user_list: Vec<GetAllUser> = users
			.into_iter()
			.map(|user| GetAllUser {
				id: user.id,
				username: user.username,
				email: user.email,
				phone: user.phone.map(|phonenumer| phonenumer.to_string()),
				created_at: user.created_at,
				updated_at: user.updated_at,
				is_active: user.is_active,
				soft_delete: user.soft_delete,
			})
			.collect();
		Ok(Json(user_list))
		}
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}
```
- **Khởi tạo**
    
    - Hàm `get_all_user` được sử dụng để lấy danh sách tất cả người dùng từ cơ sở dữ liệu và trả về dưới dạng JSON.
    - `Extension(db)`: Lấy tham số `db` kiểu `Arc<DatabaseConnection>` từ extension.

- **Kết nối cơ sở dữ liệu**
    
    - `let conn_db = db.as_ref();`: Tạo một biến tham chiếu `conn_db` để kết nối với cơ sở dữ liệu.

- **Truy vấn và ánh xạ dữ liệu người dùng**
    
    - `match users::Entity::find().all(conn_db).await`: Thực hiện truy vấn để lấy tất cả người dùng từ cơ sở dữ liệu.
    - Nếu truy vấn thành công (`Ok(users)`):
        - Sử dụng `.into_iter()` để lặp qua từng người dùng trong danh sách kết quả.
        - `user_list: Vec<GetAllUser>`: Tạo một danh sách các đối tượng `GetAllUser`.
        - `users.into_iter().map(|user| GetAllUser { ... }).collect()`: Ánh xạ mỗi đối tượng người dùng (`user`) thành một đối tượng `GetAllUser`, bao gồm các trường `id`, `username`, `email`, `phone`, `created_at`, `updated_at`, `is_active`, và `soft_delete`.
        - `phone: user.phone.map(|phone_number| phone_number.to_string())`: Chuyển đổi số điện thoại thành chuỗi (nếu có).

- **Trả về kết quả**

    - Nếu truy vấn thành công, trả về danh sách người dùng dưới dạng JSON (`Ok(Json(user_list))`).
    - Nếu truy vấn thất bại, trả về mã lỗi `StatusCode::INTERNAL_SERVER_ERROR`.

![[GetAll.png]]

>[!NOTE]
>- Trong đoạn code của mình có thêm Arc vào phần khai báo khi kết nối với CSDL. Arc (viết tắt của **Atomic Reference Counting**) là một smart pointer trong Rust được sử dụng để chia sẻ dữ liệu an toàn giữa nhiều luồng. 
>- Nó giúp quản lý bộ đếm tham chiếu một cách nguyên tử, đảm bảo rằng dữ liệu chỉ bị giải phóng khi không còn bất kỳ tham chiếu nào tới nó. Điều này rất hữu ích trong lập trình đa luồng, nơi mà việc chia sẻ dữ liệu có thể dẫn đến lỗi dữ liệu (data race) nếu không được quản lý cẩn thận.

------------------------------------------------------------------------
##### **Model**
=> Model
```rust 
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
  
#[derive(Serialize,Deserialize)]
pub struct APIResponse{
	pub message:String,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Users{
	pub id: i32,
	pub username: Option<String>,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
	pub password: Option<String>,

}

#[derive(Serialize,Deserialize)]
pub struct CreateRequest{
	pub email:String,
	pub password:String,
}

#[derive(Serialize,Deserialize)]
pub struct LoginReq{
	pub email:String,
	pub password:String,
}

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
	pub token: String,
	pub message: String,
}

#[derive(Serialize,Deserialize)]
pub struct EditUserReq{
	pub new_username: String,
	pub new_email: String,
	pub new_phone: i32,
	pub updated_at: Option<NaiveDateTime>,
	pub new_password: String,
}

#[derive(Serialize,Deserialize)]
pub struct GetAllUser{
	pub id:i32,
	pub username: Option<String>,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
	pub is_active:Option<bool>,
	pub soft_delete:Option<bool>,
}
```

**Giải thích về `Option<T>`**

Khi sử dụng `Option<bool>` trong các struct, bạn có thể biểu diễn ba trạng thái:
	 **Some(true)**: Người dùng đang hoạt động.
	 **Some(false)**: Người dùng không hoạt động.
	 **None**: Trạng thái hoạt động của người dùng không xác định.

**Giải thích về `Serialize` và `Deserialize`**

- **Serialize**: Macro `Serialize` từ thư viện `serde` được sử dụng để chuyển đổi (serialize) một cấu trúc dữ liệu thành một định dạng khác như JSON hoặc XML. Việc này thường hữu ích khi bạn muốn gửi dữ liệu qua mạng hoặc lưu trữ nó ở đâu đó. Khi một struct được đánh dấu với `#[derive(Serialize)]`, `serde` sẽ tự động tạo ra mã cần thiết để chuyển đổi struct đó sang định dạng khác.

- **Deserialize**: Macro `Deserialize` từ thư viện `serde` được sử dụng để chuyển đổi (deserialize) một định dạng khác như JSON hoặc XML trở lại thành một cấu trúc dữ liệu. Điều này cho phép bạn nhận dữ liệu từ các nguồn bên ngoài và sử dụng nó dưới dạng các struct của Rust. Khi một struct được đánh dấu với `#[derive(Deserialize)]`, `serde` sẽ tự động tạo ra mã cần thiết để chuyển đổi dữ liệu từ định dạng khác về struct đó.

>[!TIP]
> **Serialize** : Code **Rust** => **JSON** hoặc **XML** file
> **Deserialize** : **JSON, XML**  => Code **Rust**

------------------------------------------------------------------------
##### **Midderware**

***JWT (JSON Web Token)***
- JWT (JSON Web Token) là một tiêu chuẩn mở được sử dụng để chia sẻ thông tin một cách an toàn giữa các bên dưới dạng JSON. Thông tin trong JWT được xác thực và đảm bảo tính toàn vẹn bởi vì nó được ký số. JWT thường được sử dụng để xác thực và ủy quyền trong các ứng dụng web.

- Một JWT bao gồm ba phần, được phân tách bằng dấu chấm (`.`):

	1. **Header**: Chứa thông tin về thuật toán ký số và loại token.
	2. **Payload**: Chứa các claims (dữ liệu) mà bạn muốn chia sẻ.
	3. **Signature**: Được tạo bằng cách mã hóa header và payload với một khóa bí mật hoặc khóa công khai.

***Claims***

- Claims là các thông tin (dữ liệu) được chứa trong phần payload của JWT. Claims có thể chứa bất kỳ thông tin nào mà bạn muốn chia sẻ giữa các bên
- Trong đoạn Rust của tôi, claims được định nghĩa như sau:
	- `exp`: Thời gian hết hạn của token.
	- `sub`: Đối tượng mà token đại diện.Ở đây là ID người dùng
	- `username`: Tên người dùng.

>[!NOTE]
	>Bạn có thể tùy biến trong Claims để có thể trả về kết quả mong muốn. Ví dụ banj có thể thêm **fullname** hay **email** vào ***struct Claims***  

***Sử dụng JWT và Claims trong Ứng dụng***

JWT thường được sử dụng trong các hệ thống xác thực và ủy quyền như sau:

1. **Xác thực (Authentication)**: Khi người dùng đăng nhập thành công, hệ thống phát hành một JWT chứa các claims về người dùng. Token này sau đó được gửi đến khách hàng (client) và được lưu trữ (ví dụ: trong cookie hoặc local storage).
    
2. **Ủy quyền (Authorization)**: Khi khách hàng gửi yêu cầu tới máy chủ, JWT được gửi kèm trong header của yêu cầu. Máy chủ kiểm tra token để xác thực và trích xuất các claims để xác định quyền truy cập của người dùng.

=> **Midderware**

```rust
use std::env;
use crate::helpers::api_error::APIerror;
use ::entity::users::Entity;
use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use entity::users::Column;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Claims {
	exp: usize,
	sub: String,
	pub username: String,
}


pub async fn guards<T>(req: Request<T>, next: Next) -> Result<Response, APIerror> {
	let token = req
		.headers()
		.get("Authorization")
		.ok_or_else(|| {
	let error_message = "No Auth token found".to_owned();
	println!("Error: {}", error_message);
	APIerror {
		message: error_message,
		status_code: StatusCode::BAD_REQUEST,
		}
	})?
	.to_str()
	.map_err(|err| {
		let error_message = format!("Failed to convert token to string. Error: {:?}", err);
		println!("{}", error_message);
		APIerror {
			message: error_message,
			status_code: StatusCode::BAD_REQUEST,
			}
		})?.trim();

	if !token.starts_with("Bearer ") {
	let error_message = "Authorization header must start with Bearer".to_owned();
		println!("Error: {}", error_message);
		return Err(APIerror {
			message: error_message,
			status_code: StatusCode::BAD_REQUEST,
		});
	}
	let token = &token[7..];
	let claim = verify_token(token).map_err(|err| {
		println!("Error verifying JWT: {:?}", err);
		APIerror {
			message: "Unauthorized".to_owned(),
			status_code: StatusCode::UNAUTHORIZED,
		}
	})?;


	let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let db = Database::connect(db_url).await.unwrap();
  
	let _identity = Entity::find()
		.filter(Column::Username.eq(claim.username.to_lowercase()))
		.one(&db)
		.await
		.unwrap();

	let req = req.map(|_| Body::empty());
	Ok(next.run(req).await)
}
```

**1.** **Hàm tạo JWT token**
```rust
pub fn create_jwt(id: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
	let exp = chrono::Utc::now()
			.checked_add_signed(chrono::Duration::hours(2))
			.expect("valid timestamp")
			.timestamp();
	let my_secret_key: String = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY must be set");
	let claims = Claims {
		sub: id.to_owned(),
		exp: exp as usize,
		username: username.to_owned(),
	};
	let token = encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(my_secret_key.as_ref()),
		)?;
	   Ok(token)
	}
```

- **Khởi tạo**
    
    - Hàm `create_jwt` được sử dụng để tạo một JSON Web Token (JWT) chứa thông tin người dùng.
    - Tham số đầu vào:
        - `id`: ID của người dùng, kiểu `&str`.
        - `username`: Tên người dùng, kiểu `&str`.

- **Tạo thời gian hết hạn cho JWT**

    - `let exp = chrono::Utc::now().checked_add_signed(chrono::Duration::hours(2)).expect("valid timestamp").timestamp();`
        - Lấy thời gian hiện tại theo UTC.
        - Cộng thêm 2 giờ vào thời gian hiện tại.
        - Chuyển đổi thời gian thành một giá trị timestamp (số giây kể từ thời điểm 1/1/1970).

- **Lấy khóa bí mật từ biến môi trường**
    
    - `let my_secret_key: String = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY must be set");`
        - Lấy giá trị của biến môi trường `MY_SECRET_KEY`.
        - Đảm bảo rằng biến môi trường này đã được thiết lập, nếu không sẽ gây lỗi.
- **Tạo struct Claims**
    
    - `let claims = Claims { sub: id.to_owned(), exp: exp as usize, username: username.to_owned(), };`
        - `sub`: Đặt là ID của người dùng.
        - `exp`: Đặt là thời gian hết hạn.
        - `username`: Đặt là tên người dùng.

- **Mã hóa Claims thành JWT**
    
    - `let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(my_secret_key.as_ref()))?;`
        - Sử dụng thư viện `jsonwebtoken` để mã hóa `Claims`.
        - Sử dụng header mặc định cho JWT.
        - Truyền struct `Claims` vào để mã hóa.
        - Sử dụng khóa bí mật để mã hóa JWT.
        
- **Trả về kết quả**
    
    - `Ok(token)`
        - Trả về chuỗi JWT vừa được mã hóa.

**2. ** **Verity JWT**
```rust
fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
	let my_secret_key: String = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY must be set");
	let validation = Validation::default();
		decode::<Claims>(
			&token,
			&DecodingKey::from_secret(my_secret_key.as_ref()),
			&validation,
		)
		.map(|data| data.claims)
	}
```

- **Khởi tạo**
    
    - Hàm `verify_token` được sử dụng để xác minh một JSON Web Token (JWT) và trích xuất thông tin người dùng từ token đó.
    - Tham số đầu vào:
        - `token`: Chuỗi JWT cần xác minh, kiểu `&str`.

- **Lấy khóa bí mật từ biến môi trường**
    
    - `let my_secret_key: String = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY must be set");`
        - Lấy giá trị của biến môi trường `MY_SECRET_KEY`.
        - Đảm bảo rằng biến môi trường này đã được thiết lập, nếu không sẽ gây lỗi.

- **Thiết lập cấu hình xác minh**
    
    - `let validation = Validation::default();`
        - Sử dụng cấu hình mặc định để xác minh token.

- **Xác minh và giải mã token**
    
    - `decode::<Claims>( &token, &DecodingKey::from_secret(my_secret_key.as_ref()), &validation, )`
        - Sử dụng thư viện `jsonwebtoken` để giải mã và xác minh JWT.
        - `&token`: JWT cần xác minh.
        - `&DecodingKey::from_secret(my_secret_key.as_ref())`: Khóa bí mật được sử dụng để giải mã JWT.
        - `&validation`: Cấu hình xác minh mặc định.

- **Trả về kết quả**
    
    - `.map(|data| data.claims)`
        - Nếu xác minh thành công, trích xuất và trả về thông tin người dùng từ claims.
        - Nếu xác minh thất bại, trả về lỗi từ thư viện `jsonwebtoken`.
**3. ** ***Guards***

```rust
pub async fn guards<T>(req: Request<T>, next: Next) -> Result<Response, APIerror> {
	let token = req
		.headers()
		.get("Authorization")
		.ok_or_else(|| {
	let error_message = "No Auth token found".to_owned();
	println!("Error: {}", error_message);
	APIerror {
		message: error_message,
		status_code: StatusCode::BAD_REQUEST,
		}
	})?
	.to_str()
	.map_err(|err| {
		let error_message = format!("Failed to convert token to string. Error: {:?}", err);
		println!("{}", error_message);
		APIerror {
			message: error_message,
			status_code: StatusCode::BAD_REQUEST,
			}
		})?.trim();

	if !token.starts_with("Bearer ") {
	let error_message = "Authorization header must start with Bearer".to_owned();
		println!("Error: {}", error_message);
		return Err(APIerror {
			message: error_message,
			status_code: StatusCode::BAD_REQUEST,
		});
	}
	let token = &token[7..];
	let claim = verify_token(token).map_err(|err| {
		println!("Error verifying JWT: {:?}", err);
		APIerror {
			message: "Unauthorized".to_owned(),
			status_code: StatusCode::UNAUTHORIZED,
		}
	})?;


	let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let db = Database::connect(db_url).await.unwrap();
  
	let _identity = Entity::find()
		.filter(Column::Username.eq(claim.username.to_lowercase()))
		.one(&db)
		.await
		.unwrap();

	let req = req.map(|_| Body::empty());
	Ok(next.run(req).await)
}
```

- **Khởi tạo**
    
    - Hàm `guards` được sử dụng để xác minh token JWT trong tiêu đề yêu cầu HTTP và kiểm tra người dùng trong cơ sở dữ liệu.
    - Tham số đầu vào:
        - `req`: Yêu cầu HTTP, kiểu `Request<T>`.
        - `next`: Middleware tiếp theo, kiểu `Next`.

- **Lấy và kiểm tra tiêu đề `Authorization`**
    
    - `let token = req.headers().get("Authorization")`: Lấy tiêu đề `Authorization` từ yêu cầu.
    - `ok_or_else(|| { ... })?`: Nếu không tìm thấy tiêu đề `Authorization`, trả về lỗi `BAD_REQUEST`.
    - `to_str().map_err(|err| { ... })?.trim()`: Chuyển đổi giá trị của tiêu đề `Authorization` thành chuỗi, nếu chuyển đổi thất bại, trả về lỗi `BAD_REQUEST`.

- **Kiểm tra định dạng token**
    
    - `if !token.starts_with("Bearer ")`: Kiểm tra tiền tố `Bearer` trong token.
    - `let token = &token[7..];`: Bỏ tiền tố "Bearer " để lấy token thực tế.

- **Xác minh token**
    
    - `let claim = verify_token(token).map_err(|err| { ... })?`: Gọi hàm `verify_token` để xác minh JWT, nếu xác minh thất bại, trả về lỗi `UNAUTHORIZED`.

- **Kết nối cơ sở dữ liệu**
    
    - `let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");`: Lấy URL cơ sở dữ liệu từ biến môi trường `DATABASE_URL`.
    - `let db = Database::connect(db_url).await.unwrap();`: Kết nối với cơ sở dữ liệu bằng URL đã lấy.

- **Truy vấn và kiểm tra người dùng**
    
    - `let _identity = Entity::find().filter(Column::Username.eq(claim.username.to_lowercase())).one(&db).await.unwrap();`: Truy vấn cơ sở dữ liệu để tìm người dùng với tên đăng nhập từ claims.

- **Tiếp tục xử lý yêu cầu**
    
    - `let req = req.map(|_| Body::empty());`: Làm trống nội dung yêu cầu để tiếp tục xử lý.
    - `Ok(next.run(req).await)`: Chuyển yêu cầu đến middleware tiếp theo và trả về kết quả.



![[JWT.png]]
**Tạo JWT**

![[CheckJWT.png]]


 **Decode xem kết quả của JWT**

![[Put JWT in Header.png]]
**Gáng JWT để chạy các hàm dưới**

![[Put JWT in Header.png]]
------------------------------------------------------------------------
##### **Router**
```rust
//add extension to match with controller can check sync connect database

pub fn user_router(db : Extension<Arc<DatabaseConnection>>) -> Router {
	let pub_router = Router::new()
		.route("/v1/api/create_user", post(create_user).layer(db.clone()))
		.route("/v1/api/login_user", post(login_user).layer(db.clone()));
		
//need verify JWT token to login
	let priv_router = Router::new()
		.route("/v1/api/update_user/:id", post(update_user))
		.route("/v1/api/delete_user/:id", post(delete_user))
		.route("/v1/api/get_all_user", get(get_all_user))
		.layer(middleware::from_fn(guards));
		
	Router::new()
		.merge(pub_router)
		.merge(priv_router)
}
```

**Khởi tạo:**

- Định nghĩa hàm `user_router` nhận một tham số `db` là một extension chứa kết nối đến cơ sở dữ liệu.
- Tạo một `Router` mới để xử lý các yêu cầu từ client.

**Tạo routes cho phần công cộng (public routes):**

- Tạo một `Router` mới (`pub_router`) để quản lý các routes có thể truy cập công khai mà không cần xác thực.
- Thêm các routes như `/v1/api/create_user` và `/v1/api/login_user` và ánh xạ chúng tới các hàm xử lý tương ứng (như `create_user` và `login_user`) bằng cách sử dụng phương thức HTTP POST.
- Mỗi route cũng sẽ được layer với `db.clone()` để đảm bảo rằng chúng có thể truy cập vào cơ sở dữ liệu.

**Tạo routes cho phần riêng tư (private routes):**

- Tạo một `Router` mới (`priv_router`) để quản lý các routes yêu cầu xác thực trước khi truy cập.
- Thêm các routes như `/v1/api/update_user/:id`, `/v1/api/delete_user/:id`, và `/v1/api/get_all_user` và ánh xạ chúng tới các hàm xử lý tương ứng.
- Các routes này yêu cầu xác thực bằng JWT token, do đó chúng sẽ được layer với middleware `guards` để kiểm tra token.

**Kết hợp các routes:**

- Kết hợp cả `pub_router` và `priv_router` vào một `Router` chính bằng cách sử dụng phương thức `merge`.
- Cuối cùng, trả về `Router` đã được kết hợp, sẵn sàng để sử dụng trong ứng dụng của bạn.

>[!NOTE]
>Chỉ những endpoint có JWT và được xác mình mới có thể vào trong để thực hiện tiếp 
>

------------------------------------------------------------------------
# PostMan

Postman https://www.postman.com/phuclpst0125/workspace/webaxum/collection/30339890-2c997052-ac25-49b6-82d7-3aaca8b3222a
# Git
Git : https://github.com/PhucLam202/Rust-Axum-SeaORM-Mvc.git 
# Tổng Kết

Trong bài viết này ta đã:

- Hiểu cơ bản về web-service với axum
- Tương tác với cơ sở dữ liệu sử dụng sea-orm
- Nắm được các khái niệm cơ bản của sea-orm

*Mình chưa có quá nhiều kinh nghiệm với Rust trong việc xây dựng Backend. Trong bài viết có sai sót gì mọi người cùng thảo luận góp ý nhé.*

**Cảm ơn mọi người đã đọc.**