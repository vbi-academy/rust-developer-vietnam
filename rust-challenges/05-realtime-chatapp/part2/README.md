# Xây dựng real time chat app (P2)

Xin chào mọi người, trong bài viết lần trước thì mình đã làm được

- Setup webserver sử dụng Axum
- Tổ chức source code theo hướng feature
- Cài đặt thành công chức năng đăng nhập với JWT và tạo mới user

Trong bài này, mình sẽ hướng dẫn mọi người:

- Thiết lập cơ sở dữ liệu với Docker
- Tương tác với cơ sở dữ liệu sử dụng ORM

# Kiến thức yêu cầu

- Biết cơ bản về Docker (Docker Compose,…)
- Biết cơ bản về SQL

# Kiến thức đạt được sau bài viết này

- Thiết kế database
- Biết sử dụng sea-orm
- Biết thêm một vài khái niệm mới của framework Axum như shared state giữa các handler function,…

# Hướng dẫn

## Chạy Docker

Trong bài viết này mình sẽ không nói quá chi tiết về Docker. Nếu mọi người chưa biết về Docker, mình recommend mọi người 2 video sau mình thấy minh họa khá trực quan và dễ hiểu:

- [https://www.youtube.com/watch?v=pg19Z8LL06w](https://www.youtube.com/watch?v=pg19Z8LL06w)
- [https://www.youtube.com/watch?v=SXwC9fSwct8](https://www.youtube.com/watch?v=SXwC9fSwct8)

Mọi người tạo file `docker-compose.yaml` ở thư mục gốc nhé

```rust
version: "2.26.1"

services:
  postgres-db:
    image: postgres
    ports:
      - 5432:5432
    environment:
      - POSTGRES_PASSWORD=admin
      - POSTGRES_USER=admin
    volumes:
      - postgres-db-data:/var/lib/postgresql/data

  pgadmin4:
    image: dpage/pgadmin4
    ports:
      - 5050:80
    environment:
      - PGADMIN_DEFAULT_EMAIL=admin@gmail.com
      - PGADMIN_DEFAULT_PASSWORD=admin
    depends_on:
      - "postgres-db"

volumes:
  postgres-db-data:
```

Mình chạy 2 service

- postgres-db: database server - port 5432
- pgadmin4: UI quản lý postgres db - port 5050

Để chạy các service ta dùng lệnh sau

```bash
docker-compose -f docker-compose.yaml up
```

Sau khi chạy service nếu log của terminal hiển thị như vậy tức là các service đã chạy thành công

![Untitled](img/Untitled.png)

Ta truy cập địa chỉ [`http://localhost:5050/`](http://localhost:5050/) và nhập các trường dữ liệu sau

```
Email: admin@gmail.com
Password: admin
```

![Untitled](img/Untitled%201.png)

Sau đó ta nhấn vào `Add New Server` thì có 1 cửa sổ popup hiện lên

![Untitled](img/Untitled%202.png)

Ta nhập `Name` (tên nào cũng được nhé).

Sau đó qua tab `Connection`

![Untitled](img/Untitled%203.png)

Ở tab này bạn phải nhập các trường dữ liệu sau:

- Host name/address: postgres-db (service name trong docker-compose.yaml)
- Username: admin (set trong docker-compose.yaml)
- Password: admin (set trong docker-compose.yaml)

Kết quả sau khi nhấn `Save`

![Untitled](img/Untitled%204.png)

Tiếp theo ta tiến hành tạo database mới mà ta sẽ sử dụng cho project

Nhấn chuột phải vào icon Databases → Create → Database….

![Untitled](img/Untitled%205.png)

Kết quả sau khi nhấn `Save`

![Untitled](img/Untitled%206.png)

## Thiết kế database

[https://dbdiagram.io/](https://dbdiagram.io/): Đây là trang web hỗ trợ vẽ diagram khá tốt mà mình muốn recommend cho mọi người

![Untitled](img/Untitled%207.png)

## Cài đặt các crate cần thiết

Ta thêm các dòng sau vào `Cargo.toml`

```toml
dotenv = "0.15.0"
sea-orm = {version = "0.12.15", features = [ "sqlx-postgres", "runtime-tokio-native-tls", "macros", "debug-print" ]}
```

- dotenv: hỗ trợ load file `.env`
- sea-orm

## SeaORM

### Giới thiệu

sea-orm là 1 thư viện hỗ trợ ánh xạ từ code Rust sang code SQL

**Table** map với object (hay là **Struct** bên Rust)

**Column** được map với các **attribute** của struct

Các khái niệm cơ bản

- **Schema:** database (tập hợp các table)
- **Entity:** table (support CRUD)
  - **EntityTrait:** cung cấp API để truy cập các thuộc tính của Entity (**Column**, **Relation**, **PrimaryKey**)
  - **Model**: struct lưu trữ tất cả thuộc tính và giá trị của chúng, read only
  - **ActiveModel**: insert, update, delete
- **Migration**: quản lý các phiên bản database
  - MigrationTable: bảng quản lý việc thay đổi các version của db

### Connection and shared state

`main.rs`

```rust
pub mod enums;
pub mod features;

mod router;
use std::env;

use axum::Extension;
use dotenv::dotenv;
use router::create_router;
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() {
    // ...

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    dotenv().ok();

    let mut opt = ConnectOptions::new(env::var("DATABASE_URL").unwrap());

    let db_connection = match Database::connect(opt).await {
        Ok(conn) => conn,
        Err(e) => panic!("{}", format!("Database connection failed: {:?}", e)),
    };

    // build our application with a route
    let app = create_router().layer(Extension(db_connection));

    // ...
}

```

- Setup `tracing_subscriber` để bắt được và hiển thị debug log
- `dotenv().ok()` : load file `.env`
- Để các handler function khác có thể truy cập biến db_connection, ta thêm middleware sau vào root router
  `.layer(Extension(db_connection))`: Extension là middleware thêm thông tin (biến db_connection) vào các request

Các bạn tạo file`.env` ở thư mục gốc

```bash
DATABASE_URL=postgres://admin:admin@localhost:5432/chat-app
```

## SeaORM - Migration

### Tạo thư mục Migration

[Setting Up Migration | SeaORM 🐚 An async & dynamic ORM for Rust (sea-ql.org)](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/#creating-migration-directory)

Mọi người làm theo hướng dẫn phía trên phần `Creating Migration Directory` thôi nha.

Sau khi làm xong mọi người truy cập vào file `migration/src/m20220101_000001_create_table.rs`

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

Mình sẽ giải thích qua một vài thành phần quan trọng

- `enum Post`: định nghĩa table gồm các attribute nào (mỗi attribute là variant của enum)
- `async fn up()`: tạo table trong database
  - Định nghĩa các Column dựa trên các attribute trong enum Post
    - Thiết lập kiểu dữ liệu
    - Thiết lập khóa chính (primary key), khóa ngoại (foreign key)
    - Thiết lập các ràng buộc (constraint)
- `async fn down()`: xóa đi table trong database

`migration/src/lib.rs`

```rust
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}

```

Khi chạy các lệnh liên quan đến migrate của `sea-orm-cli` thì sẽ gọi function này.

Sau đó mọi người chạy lệnh sau

```bash
sea-orm-cli migrate up
# hoặc
sea-orm-cli migrate fresh
```

![Untitled](img/Untitled%208.png)

Như vậy là chạy thành công nhé. Lúc này cơ sở dữ liệu của mọi người sẽ tạo các table được định nghĩa trong các file migration

Mọi người vào UI quản lý PostgreSQL để kiểm tra nhé

![Untitled](img/Untitled%209.png)

Ngoài table `post` được định nghĩa trong file migration thì có 1 table `seaql_migrations` được tạo ra (đây là MigrationTable mình có đề cập ở phần giới thiệu)

![Untitled](img/Untitled%2010.png)

Nếu mình chạy `sea-orm-cli migrate up` một lần nữa thì kết quả sẽ như thế nào?

![Untitled](img/Untitled%2011.png)

Không có gì xảy ra vì lúc này trong table `seaql_migrations` đã lưu record rằng file migration này đã được áp dụng cho cơ sở dữ liệu

Còn nếu chạy `sea-orm-cli migrate fresh` ?

![Untitled](img/Untitled%2012.png)

Như mọi người thấy thì nó sẽ xóa tất cả các table trong cơ sở dữ liệu và tiến hành chạy lại các file migration (gọi function up của mỗi file migration)

### Define Migration

Trong phần thiết kế database, ta có 6 tables ⇒ Ta cần tạo 6 file migration

```bash
sea-orm-cli migrate generate "create user"
sea-orm-cli migrate generate "create group"
sea-orm-cli migrate generate "create message"
sea-orm-cli migrate generate "create file msg"
sea-orm-cli migrate generate "create user group"
sea-orm-cli migrate generate "create conversation"
```

Mọi người nên định nghĩa table theo thứ tự table độc lập (không phụ thuộc vào table nào) rồi đến table phụ thuộc vào table khác (khóa ngoại)

Mình sẽ định nghĩa Table liên quan tới `User` trước

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Avatar).string())
                    .col(ColumnDef::new(User::IsOnline).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    Avatar,
    IsOnline,
}
```

Đoạn code này rõ ràng rồi ha. Riêng với Column id mình có thiết lập default value để tự động tạo uuid khi mình tạo mới user

**_Lưu ý_**: chỉ dành cho **Postgres**

Nếu bạn muốn set default như trên thì bạn phải create extension `uuid-ossp` để có thể sử dụng function `uuid_generate_v4()` ⇒ Mình tạo thêm 1 file migration

```rust
sea-orm-cli migrate generate "create uuid extension"
```

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Use `execute_unprepared` if the SQL statement doesn't have value bindings
        db.execute_unprepared(
            "DROP EXTENSION IF EXISTS \"uuid-ossp\"",
        )
        .await?;

        Ok(())
    }
}
```

Mình sẽ định nghĩa hết các table còn lại

`Group`

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Group {
    Table,
    Id,
    Name,
}

```

`Message`

```rust
use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

use crate::m20240520_104423_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(MessageEnum)
                    .values(MessageVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Message::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_message_user_userId")
                            .from(Message::Table, Message::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .col(
                        ColumnDef::new(Message::Type)
                            .enumeration(MessageEnum, MessageVariants::iter()),
                    )
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Message {
    Table,
    Id,
    UserId,
    Content,
    Type,
    CreatedAt,
}

#[derive(DeriveIden)]
struct MessageEnum;

#[derive(Iden, EnumIter)]
pub enum MessageVariants {
    #[iden = "File"]
    File,
    #[iden = "Image"]
    Image,
    #[iden = "Text"]
    Text,
}
```

**Lưu ý:**

- Chỉ PostgreSQL mới gọi được `create_type()`
- Column làm khóa ngoại phải có cùng kiểu dữ liệu với Column làm khóa chính mà nó tham chiều tới
- `.on_delete(ForeignKeyAction::Cascade)` cho phép bạn xóa record bên phía bảng 1 thì tất cả record bên phía bảng N sẽ bị xóa theo tương tự cho update

`FileMsg`

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FileMsg::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FileMsg::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(FileMsg::MsgId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_fileMsg_msg_msgId")
                            .from(FileMsg::Table, FileMsg::MsgId)
                            .to(FileMsg::Table, FileMsg::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(FileMsg::FileName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileMsg::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FileMsg {
    Table,
    Id,
    MsgId,
    FileName,
}

```

`Conversation`

```rust
use sea_orm_migration::prelude::*;

use crate::{m20240520_104447_create_group::Group, m20240520_104508_create_message::Message};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Conversation::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Conversation::GroupId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_conversation_group_groupId")
                            .from(Conversation::Table, Conversation::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Conversation::MsgId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_conversation_message_msgId")
                            .from(Conversation::Table, Conversation::MsgId)
                            .to(Message::Table, Message::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .name("PK_conversation")
                            .col(Conversation::GroupId)
                            .col(Conversation::MsgId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Conversation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Conversation {
    Table,
    GroupId,
    MsgId,
}
```

- Primary key gồm 2 column

`UserGroup`

```rust
use sea_orm_migration::prelude::*;

use crate::{m20240520_104423_create_user::User, m20240520_104447_create_group::Group};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserGroup::GroupId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_UserGroup_group_groupId")
                            .from(UserGroup::Table, UserGroup::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(UserGroup::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_UserGroup_user_userId")
                            .from(UserGroup::Table, UserGroup::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .name("PK_UserGroup")
                            .col(UserGroup::GroupId)
                            .col(UserGroup::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserGroup {
    Table,
    GroupId,
    UserId,
}
```

### Atomic migration (chỉ hoạt động với PostgreSQL)

Khi chạy migration failed thì database sẽ được rolled back về trạng thái cũ (do migration script thực thi trong transaction)

### Schema First or Entity First

Có 2 cách tiếp cận khi sử dụng sea-orm là `Schema First` và `Entity First`

Trong hướng dẫn này thì mình tiếp cận theo hướng `Schema First` (chạy migration để tạo ra các table trong database rồi dựa vào đó để generate ra entity)

`migration/src/lib.rs`

```rust
pub use sea_orm_migration::prelude::*;

mod m20240520_104423_create_user;
mod m20240520_104447_create_group;
mod m20240520_104508_create_message;
mod m20240520_104520_create_file_msg;
mod m20240520_104527_create_conversation;
mod m20240520_111859_create_uuid_extension;
mod m20240521_123300_create_user_group;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240520_104423_create_user::Migration),
            Box::new(m20240520_104447_create_group::Migration),
            Box::new(m20240520_104508_create_message::Migration),
            Box::new(m20240520_104520_create_file_msg::Migration),
            Box::new(m20240520_104527_create_conversation::Migration),
            Box::new(m20240520_111859_create_uuid_extension::Migration),
            Box::new(m20240521_123300_create_user_group::Migration),
        ]
    }
}
```

Lưu ý:

- Thứ tự chạy migration rất quan trọng

Sau đó mọi người chạy lệnh `sea-orm-cli migrate fresh` và sử dụng `sea-orm-cli migrate status` để kiểm tra trạng thái các migration

![Untitled](img/Untitled%2013.png)

Vậy là ta đã tạo thành công các table trong database tiếp đến ta sẽ tạo entities từ đó.

### sea-orm-cli migrate cheatsheet

```bash
sea-orm-cli migrate init # tạo thư mục migration

sea-orm-cli migrate generate NAME_OF_MIGRATION # tạo file migration

sea-orm-cli migrate up # gọi function up được định nghĩa trong các file migration

sea-orm-cli migrate fresh # xóa tất cả table trong db xong đó apply lại các migration (gọi function up của mỗi file migration)

sea-orm-cli migrate status # kiểm tra trạng thái của các migration
```

## SeaORM - Entity

### Tạo thư mục Entity

Mọi người chạy command sau ở terminal nhé:

```bash
sea-orm-cli generate entity -u postgres://admin:admin@localhost:5432/chat-app -o entity/src
```

Nó sẽ generate các entity dựa trên các table có trong database `chat-app`

![Untitled](img/Untitled%2014.png)

Mọi người tạo thêm file `entity/Cargo.toml`

```toml
[package]
name = "entity"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "entity"
path = "src/mod.rs"

[dependencies]
serde = { version = "1", features = ["derive"] }

[dependencies.sea-orm]
version = "0.12.15"
```

### Workspace Structure

Mọi người làm theo hướng dẫn phía dưới nhé phần `Workspace Structure`.

[Setting Up Migration | SeaORM 🐚 An async & dynamic ORM for Rust (sea-ql.org)](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/#workspace-structure)

### Entity Structure

Trong bài viết này, mình sẽ không đi vào phần này bởi vì tất cả các entity đã được generate sẵn. Bây giờ mọi người chỉ cần biết thực hiện CRUD thông qua API có sẵn từ entity như thế nào.

## Xây dựng API

### Các thành phần cơ bản cho CRUD

- Entity: cung cấp API để thực hiện CRUD
  - Mọi người có thể sẽ thắc mắc trong source code entity không có Entity nào được định nghĩa sao ta gọi được đó là nhờ macro `DeriveEntityModel` hỗ trợ generate entity từ `Model`
- Mỗi dòng dữ liệu trong table ứng với `Model`
- `ActiveValue`: enum để bắt được những thay đổi đối với attribute của ActiveModel
- `ActiveModel` : có tất cả attributes (fields) của `Model` và các attribute này được theo dõi bởi ActiveValue

### User

`src/features/users/handler.rs`

```rust
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;

use super::model::{CreateUser, UpdateUser, UserDTO};

use entity::user;

pub async fn create_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user_model = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(payload.password),
        is_online: Set(payload.is_online),
        ..Default::default()
    };

    user_model.insert(&db_connection).await.unwrap();

    (
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "User created successfully"
            }
        )),
    )
}

pub async fn get_user_by_id(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .unwrap()
        .unwrap();

    let result = UserDTO {
        id: user.id,
        name: user.name,
        email: user.email,
        avatar: user.avatar,
        is_online: user.is_online,
    };

    (StatusCode::CREATED, Json(result))
}

pub async fn update_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let mut user: user::ActiveModel = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .unwrap()
        .unwrap()
        .into();

    user.name = Set(payload.name.unwrap());
    user.email = Set(payload.email.unwrap());
    user.avatar = Set(payload.avatar);

    user.update(&db_connection).await.unwrap();

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "message": "User updated successfully"
            }
        )),
    )
}

pub async fn delete_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let mut user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .unwrap()
        .unwrap();

    user::Entity::delete_by_id(user.id)
        .exec(&db_connection)
        .await
        .unwrap();

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "message": "User deleted successfully"
            }
        )),
    )
}

pub async fn get_all_users(
    Extension(db_connection): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let users: Vec<UserDTO> = user::Entity::find()
        .all(&db_connection)
        .await
        .unwrap()
        .into_iter()
        .map(|user| UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
            is_online: user.is_online,
        })
        .collect();

    (StatusCode::ACCEPTED, Json(users))
}

```

Khi định nghĩa table thì mình có set default value cho vài Column nếu mình mình muốn set value default cho các attribute không được chỉ định thì dùng `..Default::default()`

`..` : destrucutring để lấy ra các thuộc tính trong struct

Tại sao không trả về `struct Model`?

1. Model chứa 1 vài thông tin quan trọng như password
2. Model không có attribute Serialize (convert Rust type sang JSON). Ta có thể thêm vào nhưng không tốt lắm vì có thể trong tương lai ta sẽ thay đổi Schema (các file migration) thì khi ta generate entity dựa trên các table được tạo trong db nhờ chạy migration sẽ mất hết

⇒ Sử dụng DTO (Data Transfer Object)

Nếu mọi người chưa biết về `into` thì mọi người nên đọc bài viết này

[Rust-Developer-Vietnam/newbie/From_Into.md at main · openedu101/Rust-Developer-Vietnam (github.com)](https://github.com/openedu101/Rust-Developer-Vietnam/blob/main/newbie/From_Into.md)

**_Lưu ý:_**

- Có những function mà mình unwrap tận 2 lần thì đây là điều không nên. Trong bài viết sau thì mình sẽ hướng dẫn mọi người xử lý lỗi (Error Handling)

`src/features/users/model.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub is_online: bool,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub is_online: bool,
}

```

`src/features/users/router.rs`

```rust
use axum::{routing::get, Router};

use super::handler::{create_user, delete_user, get_all_users, get_user_by_id, update_user};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route(
            "/:id",
            get(get_user_by_id).delete(delete_user).patch(update_user),
        )
}
```

### Group

`src/features/group/handler.rs`

```rust
use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;

use crate::features::users::model::UserDTO;

use super::model::{CreateGroup, GroupDTO};

use entity::{group, user, user_group};

pub async fn create_group(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<CreateGroup>,
) -> impl IntoResponse {
    let group_model = group::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };
    let new_group = group_model.insert(&db_connection).await.unwrap();

    let records: Vec<user_group::ActiveModel> = payload
        .user_ids
        .into_iter()
        .map(|user_id| user_group::ActiveModel {
            group_id: Set(new_group.id),
            user_id: Set(user_id),
        })
        .collect();

    user_group::Entity::insert_many(records)
        .exec(&db_connection)
        .await;

    (
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "Group created successfully"
            }
        )),
    )
}

pub async fn get_group_by_id(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let group = group::Entity::find()
        .filter(Condition::all().add(group::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .unwrap()
        .unwrap();

    let user_ids: Vec<Uuid> = user_group::Entity::find()
        .filter(Condition::all().add(user_group::Column::GroupId.eq(group.id)))
        .all(&db_connection)
        .await
        .unwrap()
        .into_iter()
        .map(|user_group_model| user_group_model.user_id)
        .collect();

    let mut users: Vec<UserDTO> = vec![];
    for user_id in user_ids.into_iter() {
        let user = user::Entity::find()
            .filter(Condition::all().add(user::Column::Id.eq(user_id)))
            .one(&db_connection)
            .await
            .unwrap()
            .unwrap();
        users.push(UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
            is_online: user.is_online,
        });
    }

    let result = GroupDTO {
        id: group.id,
        users,
    };

    (StatusCode::OK, Json(result))
}

```

Function `get_group_by_id` chưa được tối ưu lắm nếu mọi người có cách nào hay hơn thì mình cùng thảo luận nhé

`src/features/group/model.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::users::model::UserDTO;

#[derive(Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub user_ids: Vec<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct GroupDTO {
    pub id: Uuid,
    pub users: Vec<UserDTO>,
}

```

`src/features/group/router.rs`

```rust
use axum::{
    routing::{get, post},
    Router,
};

use super::handler::{create_group, get_group_by_id};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", post(create_group))
        .route("/:id", get(get_group_by_id))
}

```

### Chat

`src/features/group/model.rs`

```rust
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Chat {
    pub user_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub group_id: Uuid,
}

#[derive(Deserialize)]
pub enum MessageType {
    File,
    Text,
    Image,
}
```

Ở đây mình định nghĩa thêm enum `MessageType` mặc dù đã có `MessageEnum` được generate bởi vì `MessageEnum` không có attribute `Deserialize` nên khi đưa vào struct `Chat` sẽ bị lỗi

`src/features/chat/handler.rs`

```rust
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use entity::sea_orm_active_enums::MessageEnum;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection,
};
use serde_json::json;

use entity::{conversation, message};

use super::model::{Chat, MessageType};

pub async fn chat(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<Chat>,
) -> impl IntoResponse {
    let message_type = match payload.message_type {
        MessageType::File => MessageEnum::File,
        MessageType::Text => MessageEnum::Text,
        MessageType::Image => MessageEnum::Image,
    };

    let message_model = message::ActiveModel {
        user_id: Set(payload.user_id),
        content: Set(payload.content),
        r#type: Set(Some(message_type)),
        ..Default::default()
    };

    let message = message_model.insert(&db_connection).await.unwrap();

    let conversation_model = conversation::ActiveModel {
        group_id: Set(payload.group_id),
        msg_id: Set(message.id),
    };

    conversation_model.insert(&db_connection).await.unwrap();

    (
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "Chat created successfully"
            }
        )),
    )
}

```

`r#type`: mình vô tình đặt tên thuộc tính trùng với các keyword trong Rust. Mình chỉ cần thêm `r#` phía trước là Rust có thể phân biệt giữa keyword với tên định nghĩa cho

`src/features/group/router.rs`

```rust
use axum::{routing::post, Router};

use super::handler::chat;

pub fn get_routes() -> Router {
    Router::new().route("/", post(chat))
}

```

`src/enums/routes.rs`

```rust
const AUTH_PATH: &str = "/auth";
const USERS_PATH: &str = "/users";
const GROUP_PATH: &str = "/group";
const CHAT_PATH: &str = "/chat";

pub enum RoutePath {
    AUTH,
    USERS,
    GROUP,
    CHAT,
}

impl RoutePath {
    pub fn get_path(&self) -> &'static str {
        match self {
            RoutePath::AUTH => AUTH_PATH,
            RoutePath::USERS => USERS_PATH,
            RoutePath::GROUP => GROUP_PATH,
            RoutePath::CHAT => CHAT_PATH,
        }
    }
}

```

`src/router.rs`

```rust
use axum::Router;

use crate::{
    enums::routes::RoutePath,
    features::{
        auth::routes::get_routes as get_auth_routes, chat::routes::get_routes as get_chat_routes,
        group::routes::get_routes as get_group_routes,
        users::routes::get_routes as get_user_routes,
    },
};

pub fn create_router() -> Router {
    let auth_routes = get_auth_routes();
    let user_routes = get_user_routes();
    let group_routes = get_group_routes();
    let chat_routes = get_chat_routes();

    let api_routes = Router::new()
        .nest(RoutePath::AUTH.get_path(), auth_routes)
        .nest(RoutePath::USERS.get_path(), user_routes)
        .nest(RoutePath::GROUP.get_path(), group_routes)
        .nest(RoutePath::CHAT.get_path(), chat_routes);

    Router::new().nest("/api", api_routes)
}

```

## Github

Mọi người có thể xem source code hoàn chỉnh ở đây nhé.

[https://github.com/Learning-Tech-Workspace/learn-rust-backend](https://github.com/Learning-Tech-Workspace/learn-rust-backend)

## Postman

[https://www.postman.com/navigation-astronaut-22006281/workspace/rust](https://www.postman.com/navigation-astronaut-22006281/workspace/rust)

## Tổng kết

Trong bài viết này ta đã:

- Thiết lập cơ sở dữ liệu với Docker
- Tương tác với cơ sở dữ liệu sử dụng sea-orm
- Nắm được các khái niệm cơ bản của sea-orm

## Bài viết tiếp theo

- Xây dựng middleware
- Tiếp tục xây dựng API
- Sử dụng socket để làm real time
- Error Handling

Mình chưa có quá nhiều kinh nghiệm với Rust trong việc xây dựng Backend. Trong bài viết có sai sót gì mọi người cùng thảo luận góp ý nhé.

Cảm ơn mọi người đã đọc.
