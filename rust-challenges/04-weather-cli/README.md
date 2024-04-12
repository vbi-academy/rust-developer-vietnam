# Weather API CLI 

## Bài toán 
Viết 1 ứng dụng cli (command line interface) thông báo thông tin dự báo thời tiết tại một số địa điểm ở thời điểm quá khứ , hiện tại, tương lai 


## Các kiến thức bạn sẽ học được trong phần challenges này 
### Error Handling

Như những challenge trước có hướng dẫn cách bạn custom lỗi cho ứng dụng, đối với thử thách này bạn có thể sử dụng `thiserror` crate. Crate này khá phổ biến trong việc error handling trong rust, vì sự tiện lợi và customize dễ dàng

Ví dụ:

Thay vì chúng ta định nghĩa enum rùi dùng trait `From` hoặc `Into` hoặc một số trait khác để map enum sang `string` và ngược lại. Ta có thể sử dụng `thiserror` thông qua `macro` #[error(...)]


```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Date provided is out of range. Valid range: {0} - {1}")]
	DateOutOfRange(String, String),
	#[error("Hour is out of range. Valid range: 0 - 24")]
	HourOutOfRange,
	#[error("Reqwest error: {0:#?}")]
	Reqwest(#[from] reqwest::Error),
	#[error("Invalid date format, must be in YYYY-MM-DD")]
	InvalidDate(#[from] chrono::ParseError),
}
```

### Cách call api 

Phần này có đề cập ở những thử thách trước , các bạn có thể xem lại nhé 

### Cách sử dụng serde 

`Serde` crate cũng là 1 thư viện sử dụng rất phổ biến khi làm việc với cấu trúc dữ liệu JSON. Hiểu đơn giản là map giữa JSON type và kiểu dữ liệu trong Rust. 

Ví dụ: 
+ Định nghĩa response khi call api. Deserialize nghĩa là mình convert sang kiểu dữ liệu của Rust ( nếu map), trong trường hợp này là struct 

```rust
#[derive(Debug, Deserialize)]

pub struct FutureResponse {
	pub location: Location,
	pub forecast: Forecast,
}
```

+ Unwrap deserialize (trong trường hợp example)

```rust
let table = json_to_table(&response).with(Style::rounded()).to_string();
```

### Cách sử dụng clap
+ Clap là 1 thư viện hỗ trợ viết ứng dụng cli, hỗ trợ custom nhanh chóng và tuỳ biến bằng cách sử dụng macro

Ví dụ:

```rust
#[derive(Parser)]
#[command(name = "Rust Weather CLI tool")]
#[command(version = "1.0")]
#[command(about = "Handy dandy tool to check the weather forecast", long_about = None)]
enum WeatherCli {
	/// Check the current weather
	Current {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
	},
	/// Check the forecast weather. Going from 1 - 14 days in the future from now
	Forecast {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
		/// Number of days of weather forecast. Value ranges from 1 to 14
		#[arg(long)]
		days: u8,
		/// Date should be between today and next 14 day in yyyy-MM-dd format. e.g. '2015-01-01'
		#[arg(long)]
		date: Option<String>,
		/// Must be in 24 hour. For example 5 pm should be hour=17, 6 am as hour=6
		#[arg(long)]
		hour: Option<u8>,
		/// Enable/Disable Air Quality data in forecast API output. Example, aqi=true or aqi=false
		#[arg(long)]
		aqi: Option<bool>,
	},
	/// Check the forecast weather. Going from 14 - 300 days in the future from now
	Future {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
		/// Date should be between 14 days and 300 days from today in the future in yyyy-MM-dd format (i.e. dt=2023-01-01)
		#[arg(long)]
		date: Option<String>,
	},
	/// Check the forecast history
	History {
		/// Pass US Zipcode, UK Postcode, Canada Postalcode, IP address, Latitude/Longitude (decimal degree) or city name.
		#[arg(short, long)]
		query: String,
		/// Date on or after 1st Jan, 2015 in yyyy-MM-dd format
		#[arg(long)]
		date: String,
		/// Date on or after 1st Jan, 2015 in yyyy-MM-dd format
		/// 'end_date' should be greater than 'date' parameter and difference should not be more than 30 days between the two dates.
		#[arg(long)]
		end_date: Option<String>,
		/// Must be in 24 hour. For example 5 pm should be hour=17, 6 am as hour=6
		#[arg(long)]
		hour: Option<u8>,
	},
}
```

## Link tham khảo 
+ https://serde.rs/
+ https://www.weatherapi.com/
+ https://docs.rs/clap/latest/clap/
+ https://docs.rs/thiserror/latest/thiserror/


## Tác giả

https://github.com/VintageWander/rust-weather-cli