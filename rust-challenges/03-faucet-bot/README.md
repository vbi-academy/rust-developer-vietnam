# Faucet discord

## Hash Function : Hàm băm

Là một hàm toán học chuyển đổi dữ liệu đầu vào thành 1 chuỗi số cố định 

f(x) = y 

x: input

y: hash value 

f(): hàm băm 

Một số hàm băm phổ biến: SHA256, MD5


Đặc tính:

- Xác định được (Deterministic) : Đối với một đầu vào cụ thể, hàm băm sẽ tạo ra 1 đầu ra cố định
- Kích thước đầu ra cố định: ví dụ SHA256 → đầu ra (hash value) là 256 bit
- Không thể đảo ngược quá trình
- Kháng xung đột ( Collision Resistance): 2 giá trị đầu vào không thể tạo ra cùng 1 giá trị băm

https://emn178.github.io/online-tools/sha256.html

## Public Key: Khoá công khai

- Chia sẻ công khai
- Sử dụng để mã hoá dữ liệu
- được tạo ra từ private key

Ví dụ trong ETH:0xb9b355abf795769d9137e11377f08b82a5542fa49efde43a65c4496ca3ab866c

## Private Key : Khoá riêng tư

- Phần bí mật không chia sẻ cho bất kì ai
- Nó được sử dụng để giải mã dữ liệu đã được mã hoá bằng khoá công khai
- 32 bytes

Ví dụ trong ETH: 5ab1e5271387ec6f75b74fa4c7d1d26c924b5d79f6e8e99f0775a37abf2774b8

## Address

- Rút gọn từ public key bằng cách sử dụng hàm băm keccak-256
- Lấy 20 bytes cuối cùng sau khi dùng hàm băm

Ví dụ trong ETH: `0xb9b355abf795769d9137e11377f08b82a5542fa4`

Làm thế nào để generate private key và public key ( cặp khoá ) 

- Có thể sử dụng ví không lưu ký như Metamask, Trust Wallet để generate ra private key và public key
- Sử dụng code để có thể tự generate cho mình

https://secretscan.org/generator

## Cấu trúc cơ bản của blockchain


## Yêu cầu trước khi vào challenge

- Hiểu cơ bản liên quan tới Rust
- Hiểu cách gọi http, sử dụng serenity cơ bản  trong phần Challenge trước
- Hiểu cơ bản các khái niệm liên quan tới blockchain

## Bài toán

Faucet testnet coin cho 1 user nào đó yêu cầu 

## Thư viện hỗ trợ tương tác với blockchain

- Ethers
- Web3

## Các bước thực hiện

- Sử dụng template tạo command từ serenity-rs
- Check balance của 1 address nào đó trên Sepolia (Ethereum Testnet)
- Lấy gía hiện tại của ETH
- Viết command faucet testnet coin

Link code:
https://github.com/CocDap/Rust-Challenges/tree/main/faucet-bot
