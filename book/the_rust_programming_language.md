# **Ngôn Ngữ Lập Trình Rust**
*biên soạn bởi Steve Klabnik and Carol Nichols, cùng sự đóng góp từ cộng đồng Rust*

Một số tính năng được đề cập trong sách này yêu cầu phiên bản Rust 1.41.0 trở lên, cấu hình ```edition="2018"``` trong tập tin *Cargo.toml* của dự án để sử dụng "Rust 2018". Xem phần ["Cài đặt" ở chương 1](https://doc.rust-lang.org/book/ch01-01-installation.html) để cài đặt hoặc cập nhật Rust, và xem phần [Phụ lục E](https://doc.rust-lang.org/book/appendix-05-editions.html) mới để có thêm thông tin về các phiên bản.

Phiên bản "Rust 2018" bao gồm 1 số nâng cấp giúp cho việc sử dụng và học Rust dễ dàng hơn. Các chương sau bao gồm những nâng cấp kể trên:

- Chương 7 "Quản Lý Sự Phát Triển Của Dự Án Với Các Gói, Thư Viện, Mô-đun" gần như được viết lại hoàn toàn. Mô-đun hệ thống và cách các đường dẫn hoạt động trong phiên bản 2018 được thực hiện nhất quán hơn.
- Chương 10 có thêm những phần mới với tựa đề "Sử Dụng Trait Như Là Đối Số" và "Trả Về Dữ Liệu Được Thực Thi Trait" sẽ giải thích về cú pháp mới ```impl Trait```.
- Chương 11 có thêm 1 phần mới với tựa đề "Sử Dụng ```Result<T, E>``` Trong Việc Kiểm Thử" sẽ hướng dẫn cách viết kiểm thử phần mềm sử dụng toán tử ```?```.
- Phần "Vòng Đời Nâng Cao" trong chương 19 đã bị loại bỏ vì sự nâng cấp trình biên dịch khiến cho cấu trúc trong phần này hiếm khi xảy ra.
- Phần phụ lục D trước đó, "Các Macro", đã được mở rộng thêm phần khai báo các macro và được chuyển qua phần "Các Macro" trong Chương 19.
- Phần phụ lục A, "Những Từ Khoá" cũng giải thích về *tính năng nhận dạng thô* mới giúp cho mã nguồn viết ở "Phiên bản 2015" và "Phiên bản 2018" có thể tương thích với nhau.
- Phần phụ lục D được sửa tiêu đề thành "Những Công Cụ Phát Triển Phần Mềm Hữu Ích' và viết về những công cụ đã được phát hành gần đây để giúp bạn viết mã Rust.
- Chúng tôi đã sửa đổi 1 số lỗi nhỏ và sự diễn đạt không chính xác trong sách. Xin gửi lời cảm ơn tới các độc giả đã phản hồi cho chúng tôi!

Cần lưu ý rằng bất kỳ đoạn mã nào lặp lại trong sách *The Rust Programming Language* mà đã được biên dịch thành công trước đó thì vẫn sẽ biên dịch được dù cho không có cấu hình ```edition="2018"``` trong tập tin *Cargo.toml* của dự án, thâm chí dù cho bạn có cập nhật phiên bản của trình biên dịch Rust mà bạn đang sử dụng. Đó là sự đảm bảo về tính tương thích ngược của Rust!

Phiên bản HTML cũng đang có sẵn trực tuyến tại https://doc.rust-lang.org/stable/book và phiên bản ngoại tuyến sẽ đi kèm khi cài đặt Rust bằng ```rustup```; tại cửa sổ CLI chạy ```rustup docs --book``` để mở.

Sách cũng đang có mặt trên trang [No Starch Press](https://nostarch.com/rust) dưới dạng sách giấy và sách điện tử.