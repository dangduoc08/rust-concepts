# **Giới Thiệu**

> Chú ý: Ấn bản sách này tương tự [The Rust Programming Language][nostarch/rust] đang có sẵn dưới dạng sách in và sách điện tử từ trang [No Starch Press][nostarch]

Chào mừng tới *The Rust Programming Language*, một cuốn sách giới thiệu về Rust. Ngôn ngữ lập trình Rust giúp bạn viết phần mềm nhanh hơn và đáng tin cậy hơn. Để 1 ngôn ngữ thân thiện với người dùng nhưng vẫn có thể kiểm soát sâu tầng thấp của hệ thống thường dẫn đến xung đột trong việc thiết kế ngôn ngữ lập trình, Rust thách thức sự xung đột này. Thông qua việc cân bằng giữa các tính năng kỹ thuật mạnh mẽ và 1 trải nghiệm tuyệt vời của developer, Rust cho bạn lựa chọn để kiểm soát sâu vào tầng thâp của hệ thống (như là việc sử dụng bộ nhớ) mà không hề gặp phải những rắc rối phổ biến liên quan đến các kiểu kiểm soát như vậy.

## **Rust Giành Cho Ai**

Rust lý tưởng giảnh cho nhiều người với nhiều lý do đa dạng khác nhau. Hãy xem xét 1 số nhóm quan trọng nhất sau:

### **Nhóm Những Developer**

Rust đang chứng minh là 1 công cụ hiệu quả cho những nhóm lớn các developer với sự am hiểu về lập trình hệ thống ở mức độ khác nhau. Mã bậc thấp thường dễ bị những lỗi nhỏ, mà hầu như những ngôn ngữ lập trình khác chỉ có thể bắt được những lỗi này bằng việc kiểm tra mã kỹ lưỡng và cản thận bởi những developer đầy kihh nghiệm. Ở Rust, trình biên dịch đóng vai trò người gác cổng bằng cách từ chối biên dịch những đoạn mã có những lối khó nắm bắt này, bao gồm cả lỗi xử lý đồng thời. Bằng cách làm việc cùng với trình biên dịch, các nhóm có thể giành thời gian của họ tập trung vào việc viết logic của chương trình hơn là ngồi sửa lỗi chường trình.

Rust đồng thời cũng mang đến những công cụ phát triển phần mềm hiện đại giành cho việc lập trình hệ thống:

 - Cargo, là 1 trình quản lý các gói thư viện và công cụ build, giúp cho việc thêm, biên dịch và quản lý các gói thư viện bớt khó khăn hơn và nhất quán trong hệ sinh thái của Rust.
 - Rustfmt giúp đảm bảo sự nhất quán cách viết mã giữa các developer.
 - The Rust Language Server (RLS) hỗ trợ tích hợp vào trong Integrated Development Environment (IDE) để giúp chạy mã và xuất các thông báo lỗi.

Bằng cách sử dụng những công cụ trên và nhiều công cụ khác nằm trong hệ sinh thái của Rust, những developer có thể lập trình hệ thống tốt hơn.

### **Những Bạn Học Sinh, Sinh Viên**

Rust là ngôn ngữ lập trình giành cho học sinh, sinh viên và những người muốn tìm tòi, học hỏi về các khái niệm của hệ thống. Bằng cách sử dụng Rust, nhiều người có thể học thêm về các chủ đề như là phát triển hệ điều hành hệ thống. Cộng đông rất hoan nghênh và vui vẻ trả lời các câu hỏi của các bạn. Bằng những nỗ lực thông qua quyển sách này, nhóm phát triển Rust rất mong muốn mang những khái nệm hệ thống tới nhiều người hơn, đặc biệt là những ai chưa biết gì về lập trình.

### **Những Công Ty**

Hàng trăm công ty với quy mô lớn, nhỏ khác nhau đang sử dụng Rust cho sản phẩm của họ để thực hiện những công việc đa dạng khác nhau từ viết CLI, dịch vụ Web, công cụ DevOps, thiết bị nhúng, phân tích và chuyển tín hiệu hình ảnh, video, tiền điện tử, tin sinh học, công cụ tìm kiếm, ứng dụng IOT, máy học và thậm chí là những thành phần chính của trình duyệt Firefox.

### **Những Developer Mã Nguồn Mở**

Rust giành cho nhũng ai muốn xây dựng ngôn ngữ lập trình Rust, cộng đồng, công cụ phát triển phần mềm và những thư viện. Chúng tôi rất mong muốn bạn cùng đóng góp, phát triển cho ngôn ngữ Rust.

### **Những Ai Coi Trọng Tốc Độ Và Sự Ổn Định**

Rust giành cho những ai mong muốn tốc độ và sự ổn định trong 1 ngôn ngữ. Tốc độ ở đây tức là tốc độ của các chương trình được viết bằng Rust và tốc độ viết những chương trình bằng Rust. Sự kiểm soát của trình biên dịch Rust nhằm đảo bảo tính ổn định của chương trình xuyên suốt quá trình thêm những tính năng mới hoặc tái cấu trúc mã nguồn. This is in contrast to the brittle legacy code in languages without these checks, which developers are often afraid to modify. Bằng cách đạt được zero-cost abstract những tính năng bậc cao hơn được biên dịch sang mã bậc thấp hơn nhanh như mã được viết thủ công. Rust cố gắng khiến cho mã an toàn cũng nhanh.

## **Những Ai Nên Đọc Sách Này**

Sách này giả định bạn đã từng lập trình bằng ngôn ngữ khác nhưng sẽ không giả định chính xác đó là ngôn ngữ nào. Chúng tôi đã cố gắng làm cho tài liệu có thể tiếp cận rộng rãi tới mọi ngươi từ nhiều nền tảng lập trình khác nhau. Chúng tôi sẽ không giành thời gian để nói về lập trình là gì hoặc cách tư duy lập trình như thế nào. Nếu bạn hoàn toàn mới học lập trình, sẽ tốt hơn nếu bạn tìm đọc 1 cuốn sách giới thiệu cụ thể về lập trình.

## **Đọc Sách Này Như Thế Nào**

Nhìn chung, chúng tôi mong muốn bạn đọc sách này tuần tự từ trang đầu tới trang cuối. Các chương sau thường được xây dựng dựa trên các khái niệm của chương trước. Các chương trước thường sẽ không đi sâu vào chi tiêt của 1 chủ đề nào đó, chúng ta thường xem lại các chủ đề đó ở các chương sau.

Bạn sẽ tìm thấy 2 loại chương trong quyển sách này: những chương về khái niệm và những chương về dự án. Ở những chương về khá niêm, bạn sẽ được học về các khía cạnh của Rust. Ở những chương về dự án, chúng ta sẽ cùng nhau xây dựng những dự án nhỏ, áp dụng những gì bạn đã học cho đến nay. Chương 2, 12 và 20 là những chương về làm dự án, các chương còn lại là về khái niệm.

Chương 1, hướng dẫn cách cài dặt Rust, làm thế nào để viết 1 chương rình "Hello, world!", và cách sử dụng Cargo, trình quản lý các gói thư viện Rust và công cụ build. Chương 2 là thực hành giới thiệu về ngôn ngữ Rust. Ở đây chúng ta sẽ đi qua những khái niệm bậc cao, và những chương sau đó sẽ cung cấp thêm thông tin chi tiết. Néu bạn muốn nhúng tay vào làm ngay bây giờ thì Chương 2 là nơi giành cho bạn. Đầu tiên, bạn có thể muốn bỏ qua Chương 3, nó nói về những tính năng của Rust tương tự như những ngôn ngữ lập trình khác, và nhảy thẳng tới Chương 4 để học về hệ thống ownership của Rust. Tuy nhiên, nếu bạn là người đặc biệt thích học hỏi tỉ mỉ, bạn có thể sẽ muốn bỏ qua Chương 2 và nhảy thẳng tới Chương 3, quay trở lại Chương 2, khi bạn muốn những gì bạn vừa được học vào trong 1 dự án.

Chương 5 bàn luận về kiểu dữ liệu struct và các method, và Chương 6 viết về enum, biểu thức ```match```, và cấu trúc điều khiển luồng ```if let```. Bạn sẽ sử dụng struct và enum để tuỷ chỉnh kiểu dữ liệu trong Rust.

Chương 7, bạn sẽ học về các mô-đun hệ thống của Rust và các quy luật ngầm tổ chức mã nguồn của bạn và các Giao Diện Ứng Dụng Lập Trình (API) của nó. Chương 8 thảo luận về 1 số cấu trúc dữ liệu tập hợp phổ biến mà thư viện tiêu chuẩn cung cấp, như là vector, chuỗi, bảng băm. Chương 9 khám phá về triết lý và kỹ thuật xử lý lỗi trong Rust.

Chương 10, đào sâu vào generic, trait và vòng đời, cung cấp cho bạn khả năng để định nghĩa mã ứng dụng cho nhiều loại

Chương 13, khám phá về closure và vòng lặp: những tính năng của Rust đến tư những ngôn ngữ lập trình hàm. Trong Chương 14, chúng ta sẽ xem xét Cargo sâu hơn và nói về cách tốt nhất để chia sẻ những thư viện của bạn với người khác.Chương 15 thảo luận về con trỏ thông minh thư viện tiêu chuẩn cung cấp và chức năng của nó.

Trong Chương 16, chúng ta sẽ đi qua những mô hình lập trình đồng thời khác nhau và nói về cách Rust giúp bạn lập trình đa luồng 1 cách không sợ hãi. Chương 17 xem xét các thành ngữ của Rust so với các nguyên tắc lập trình hướng đối tượng mà bạn có thể cảm thấy thân quen.

Chương 18 là 1 tài liệu tham khảo về khuôn mẫu và pattern matching, đây là 1 cách mạnh mẽ để diễn đặt những chương trình Rust. Chương 19 bao gồm 1 loạt chủ đề nâng cao thú vị như là Rust không an toàn, macro, nói nhiều hơn về vòng đời, trait, các kiểu, các hàm và closures.

Chương 20, chúng ta sẽ hoàn thành 1 dự án, chúng ta sẽ thực hiện 1 máy chủ web đa luồng bậc thấp.

Cuối cùng, 1 số phụ lục chứa các thông tin hữu ích về ngôn ngữ ở dạng giống tài liệu tham khảo. Phụ lục A bao gồm các từ khoả của Rust. Phụ lục B bao gồm các toán tử và các ký hiệu của Rust, phụ lục C


| Ferris                                        | Ý nghĩa                                    |
|-----------------------------------------------|--------------------------------------------|
| ![Does not compile][does_not_compile]         | Mã này không biên dịch được!               |
| ![Not desired behavior][not_desired_behavior] | Mã này có gì đó không đúng!                |
| ![Panics][panics]                             | Đoạn mã này chứa mã không an toàn.         |
| ![Unsafe][unsafe]                             | Mã này không tạo ra hành vi như mong muốn. |


[nostarch]: https://nostarch.com
[nostarch/rust]: https://nostarch.com/rust

[does_not_compile]: assets/does_not_compile.svg
[not_desired_behavior]: assets/not_desired_behavior.svg
[panics]: assets/panics.svg
[unsafe]: assets/unsafe.svg