-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Máy chủ: 127.0.0.1
-- Thời gian đã tạo: Th3 11, 2025 lúc 01:47 PM
-- Phiên bản máy phục vụ: 10.4.32-MariaDB
-- Phiên bản PHP: 8.2.12

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Cơ sở dữ liệu: `storages_documents`
--

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `account`
--

CREATE TABLE `account` (
  `ID` varchar(30) NOT NULL,
  `Username` varchar(100) DEFAULT NULL,
  `FullName` varchar(255) DEFAULT NULL,
  `Password` varchar(255) DEFAULT NULL,
  `Email` varchar(100) DEFAULT NULL,
  `PhoneNumber` varchar(20) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `account`
--

INSERT INTO `account` (`ID`, `Username`, `FullName`, `Password`, `Email`, `PhoneNumber`) VALUES
('ND01', 'nguyenxuanphuc', 'Nguyễn Xuân Phúc', '123456', 'phuc@gmail.com', NULL),
('ND02', 'PhamGiaBao', 'Phạm Gia Bảo', '123456', 'bao@gmail.com', NULL);

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `files`
--

CREATE TABLE `files` (
  `FileCode` varchar(50) NOT NULL,
  `Title` text DEFAULT NULL,
  `Maintenance` varchar(30) DEFAULT NULL,
  `Rights` varchar(30) DEFAULT NULL,
  `Creator` varchar(3) DEFAULT NULL,
  `StartDate` date DEFAULT NULL,
  `EndDate` date DEFAULT NULL,
  `DocToTal` decimal(10,2) DEFAULT NULL,
  `PageTotal` decimal(10,2) DEFAULT NULL,
  `OranId` varchar(13) DEFAULT NULL,
  `FileNoNation` varchar(20) DEFAULT NULL,
  `TypeId` varchar(10) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `files`
--

INSERT INTO `files` (`FileCode`, `Title`, `Maintenance`, `Rights`, `Creator`, `StartDate`, `EndDate`, `DocToTal`, `PageTotal`, `OranId`, `FileNoNation`, `TypeId`) VALUES
('HS01', 'Nghị định 30/2020/NĐ-CP về công tác văn thư', NULL, NULL, NULL, '2020-03-05', NULL, NULL, NULL, 'CQ01', '30/2020/NĐ-CP', 'ND');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `infomation_documents_arrival`
--

CREATE TABLE `infomation_documents_arrival` (
  `InfoId` varchar(50) DEFAULT NULL,
  `ArrivalDate` date DEFAULT NULL,
  `ArrivalNumber` decimal(10,2) DEFAULT NULL,
  `TraceHeaderList` text DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `infomation_documents_out`
--

CREATE TABLE `infomation_documents_out` (
  `InfoId` varchar(50) NOT NULL,
  `FileCatalog` varchar(4) DEFAULT NULL,
  `Subject` text DEFAULT NULL,
  `CodeNumber` varchar(11) DEFAULT NULL,
  `LanId` varchar(10) DEFAULT NULL,
  `TypeId` varchar(10) DEFAULT NULL,
  `PageAmount` decimal(10,2) DEFAULT NULL,
  `Receives` varchar(255) DEFAULT NULL,
  `Description` text DEFAULT NULL,
  `DueDate` date DEFAULT NULL,
  `FileCode` varchar(50) DEFAULT NULL,
  `ValidityStatus` varchar(13) DEFAULT NULL,
  `CodeNotation` varchar(30) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `infomation_documents_out`
--

INSERT INTO `infomation_documents_out` (`InfoId`, `FileCatalog`, `Subject`, `CodeNumber`, `LanId`, `TypeId`, `PageAmount`, `Receives`, `Description`, `DueDate`, `FileCode`, `ValidityStatus`, `CodeNotation`) VALUES
('TT01', '2020', 'NGHỊ ĐỊNH Về công tác văn thư \r\nCăn cứ Luật Tổ chức Chính phủ ngày 19 tháng 6 năm 2015;\r\nTheo đề nghị của Bộ trưởng Bộ Nội vụ;\r\nChính phủ ban hành Nghị định về công tác văn thư.\r\n\r\nChương I QUY ĐỊNH CHUNG\r\nĐiều 1. Phạm vi điều chỉnh\r\nNghị định này quy định về công tác văn thư và quản lý nhà nước về công tác văn thư. Công tác văn thư được quy định tại Nghị định này bao gồm: Soạn thảo, ký ban hành văn bản; quản lý văn bản; lập hồ sơ và nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan; quản lý và sử dụng con dấu, thiết bị lưu khóa bí mật trong công tác văn thư.\r\nĐiều 2. Đối tượng áp dụng\r\n1. Nghị định này áp dụng đối với cơ quan, tổ chức nhà nước và doanh nghiệp nhà nước (sau đây gọi chung là cơ quan, tổ chức).\r\n2. Tổ chức chính trị, tổ chức chính trị - xã hội, tổ chức xã hội, tổ chức xã hội - nghề nghiệp căn cứ quy định của Nghị định này và các quy định của Đảng, của pháp luật có liên quan để áp dụng cho phù hợp.\r\nĐiều 3. Giải thích từ ngữ\r\nTrong Nghị định này, những từ ngữ dưới đây được hiểu như sau:\r\n1. “Văn bản” là thông tin thành văn được truyền đạt bằng ngôn ngữ hoặc ký hiệu, hình thành trong hoạt động của các cơ quan, tổ chức và được trình bày đúng thể thức, kỹ thuật theo quy định.\r\n2. “Văn bản chuyên ngành” là văn bản hình thành trong quá trình thực hiện hoạt động chuyên môn, nghiệp vụ của một ngành, lĩnh vực do người đứng đầu cơ quan quản lý ngành, lĩnh vực quy định.\r\n3. “Văn bản hành chính” là văn bản hình thành trong quá trình chỉ đạo, điều hành, giải quyết công việc của các cơ quan, tổ chức.\r\n4. “Văn bản điện tử” là văn bản dưới dạng thông điệp dữ liệu được tạo lập hoặc được số hóa từ văn bản giấy và trình bày đúng thể thức, kỹ thuật, định dạng theo quy định.\r\n5. “Văn bản đi” là tất cả các loại văn bản do cơ quan, tổ chức ban hành.\r\n6. “Văn bản đến” là tất cả các loại văn bản do cơ quan, tổ chức nhận được từ cơ quan, tổ chức, cá nhân khác gửi đến.\r\n7. “Bản thảo văn bản” là bản được viết hoặc đánh máy hoặc tạo lập bằng phương tiện điện tử hình thành trong quá trình soạn thảo một văn bản của cơ quan, tổ chức.\r\n8. “Bản gốc văn bản” là bản hoàn chỉnh về nội dung, thể thức văn bản, được người có thẩm quyền trực tiếp ký trên văn bản giấy hoặc ký số trên văn bản điện tử.\r\n9. “Bản chính văn bản giấy” là bản hoàn chỉnh về nội dung, thể thức văn bản, được tạo từ bản có chữ ký trực tiếp của người có thẩm quyền.\r\n10. “Bản sao y” là bản sao đầy đủ, chính xác nội dung của bản gốc hoặc bản chính văn bản, được trình bày theo thể thức và kỹ thuật quy định.\r\n11. “Bản sao lục” là bản sao đầy đủ, chính xác nội dung của bản sao y, được trình bày theo thể thức và kỹ thuật quy định.\r\n12. “Bản trích sao” là bản sao chính xác phần nội dung của bản gốc hoặc phần nội dung của bản chính văn bản cần trích sao, được trình bày theo thể thức và kỹ thuật quy định.\r\n13. “Danh mục hồ sơ” là bảng kê có hệ thống những hồ sơ dự kiến được lập trong năm của cơ quan, tổ chức.\r\n14. “Hồ sơ” là tập hợp các văn bản, tài liệu có liên quan với nhau về một vấn đề, một sự việc, một đối tượng cụ thể hoặc có đặc điểm chung, hình thành trong quá trình theo dõi, giải quyết công việc thuộc phạm vi, chức năng, nhiệm vụ của cơ quan, tổ chức, cá nhân.\r\n15. “Lập hồ sơ” là việc tập hợp, sắp xếp văn bản, tài liệu hình thành trong quá trình theo dõi, giải quyết công việc của cơ quan, tổ chức, cá nhân theo những nguyên tắc và phương pháp nhất định.\r\n16. “Hệ thống quản lý tài liệu điện tử” là Hệ thống thông tin được xây dựng với chức năng chính để thực hiện việc tin học hóa công tác soạn thảo, ban hành văn bản; quản lý văn bản; lập hồ sơ và nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan trên môi trường mạng (sau đây gọi chung là Hệ thống).\r\n17. “Văn thư cơ quan” là bộ phận thực hiện một số nhiệm vụ công tác văn thư của cơ quan, tổ chức.\r\nĐiều 4. Nguyên tắc, yêu cầu quản lý công tác văn thư\r\n1. Nguyên tắc\r\nCông tác văn thư được thực hiện thống nhất theo quy định của pháp luật.\r\n2. Yêu cầu\r\na) Văn bản của cơ quan, tổ chức phải được soạn thảo và ban hành đúng thẩm quyền, trình tự, thủ tục, hình thức, thể thức và kỹ thuật trình bày theo quy định của pháp luật: Đối với văn bản quy phạm pháp luật được thực hiện theo quy định của Luật Ban hành văn bản quy phạm pháp luật; đối với văn bản chuyên ngành do người đứng đầu cơ quan quản lý ngành, lĩnh vực căn cứ Nghị định này để quy định cho phù hợp; đối với văn bản hành chính được thực hiện theo quy định tại Chương II Nghị định này.\r\nb) Tất cả văn bản đi, văn bản đến của cơ quan, tổ chức phải được quản lý tập trung tại Văn thư cơ quan để làm thủ tục tiếp nhận, đăng ký, trừ những loại văn bản được đăng ký riêng theo quy định của pháp luật.\r\nc) Văn bản đi, văn bản đến thuộc ngày nào phải được đăng ký, phát hành hoặc chuyển giao trong ngày, chậm nhất là trong ngày làm việc tiếp theo. Văn Bản đến có các mức độ khẩn: “Hỏa tốc”, “Thượng khẩn” và “Khẩn” (sau đây gọi chung là văn bản khẩn) phải được đăng ký, trình và chuyển giao ngay sau khi nhận được.\r\nd) Văn bản phải được theo dõi, cập nhật trạng thái gửi, nhận, xử lý.\r\nđ) Người được giao giải quyết, theo dõi công việc của cơ quan, tổ chức có trách nhiệm lập hồ sơ về công việc được giao và nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan.\r\ne) Con dấu, thiết bị lưu khóa bí mật của cơ quan, tổ chức phải được quản lý, sử dụng theo quy định của pháp luật.\r\ng) Hệ thống phải đáp ứng các quy định tại phụ lục VI Nghị định này và các quy định của pháp luật có liên quan.\r\nĐiều 5. Giá trị pháp lý của văn bản điện tử\r\n1. Văn bản điện tử được ký số bởi người có thẩm quyền và ký số của cơ quan, tổ chức theo quy định của pháp luật có giá trị pháp lý như bản gốc văn bản giấy.\r\n2. Chữ ký số trên văn bản điện tử phải đáp ứng đầy đủ các quy định của pháp luật.\r\nĐiều 6. Trách nhiệm của cơ quan, tổ chức, cá nhân đối với công tác văn thư\r\n1. Người đứng đầu cơ quan, tổ chức, trong phạm vi quyền hạn được giao có trách nhiệm chỉ đạo thực hiện đúng quy định về công tác văn thư; chỉ đạo việc nghiên cứu, ứng dụng khoa học và công nghệ vào công tác văn thư.\r\n2. Cá nhân trong quá trình theo dõi, giải quyết công việc có liên quan đến công tác văn thư phải thực hiện đúng quy định tại Nghị định này và các quy định của pháp luật có liên quan.\r\n3. Văn thư cơ quan có nhiệm vụ\r\na) Đăng ký, thực hiện thủ tục phát hành, chuyển phát và theo dõi việc chuyển phát văn bản đi.\r\nb) Tiếp nhận, đăng ký văn bản đến; trình, chuyển giao văn bản đến.\r\nc) Sắp xếp, bảo quản và phục vụ việc tra cứu, sử dụng bản lưu văn bản.\r\nd) Quản lý Sổ đăng ký văn bản.\r\nđ) Quản lý, sử dụng con dấu, thiết bị lưu khóa bí mật của cơ quan, tổ chức; các loại con dấu khác theo quy định.\r\nChương II SOẠN THẢO, KÝ BAN HÀNH VĂN BẢN HÀNH CHÍNH\r\nMục 1 THỂ THỨC, KỸ THUẬT TRÌNH BÀY VĂN BẢN HÀNH CHÍNH\r\nĐiều 7. Các loại văn bản hành chính\r\nVăn bản hành chính gồm các loại văn bản sau: Nghị quyết (cá biệt), quyết định (cá biệt), chỉ thị, quy chế, quy định, thông cáo, thông báo, hướng dẫn, chương trình, kế hoạch, phương án, đề án, dự án, báo cáo, biên bản, tờ trình, hợp đồng, công văn, công điện, bản ghi nhớ, bản thỏa thuận, giấy uỷ quyền, giấy mời, giấy giới thiệu, giấy nghỉ phép, phiếu gửi, phiếu chuyển, phiếu báo, thư công.\r\nĐiều 8. Thể thức văn bản\r\n1. Thể thức văn bản là tập hợp các thành phần cấu thành văn bản, bao gồm những thành phần chính áp dụng đối với tất cả các loại văn bản và các thành phần bổ sung trong những trường hợp cụ thể hoặc đối với một số loại văn bản nhất định.\r\n2. Thể thức văn bản hành chính bao gồm các thành phần chính\r\na) Quốc hiệu và Tiêu ngữ.\r\nb) Tên cơ quan, tổ chức ban hành văn bản.\r\nc) Số, ký hiệu của văn bản.\r\nd) Địa danh và thời gian ban hành văn bản.\r\nđ) Tên loại và trích yếu nội dung văn bản.\r\ne) Nội dung văn bản.\r\ng) Chức vụ, họ tên và chữ ký của người có thẩm quyền.\r\nh) Dấu, chữ ký số của cơ quan, tổ chức.\r\ni) Nơi nhận.\r\n3. Ngoài các thành phần quy định tại khoản 2 Điều này, văn bản có thể bổ sung các thành phần khác\r\na) Phụ lục.\r\nb) Dấu chỉ độ mật, mức độ khẩn, các chỉ dẫn về phạm vi lưu hành.\r\nc) Ký hiệu người soạn thảo văn bản và số lượng bản phát hành.\r\nd) Địa chỉ cơ quan, tổ chức; thư điện tử; trang thông tin điện tử; số điện thoại; số Fax.\r\n4. Thể thức văn bản hành chính được thực hiện theo quy định tại Phụ lục I Nghị định này.\r\nĐiều 9. Kỹ thuật trình bày văn bản\r\nKỹ thuật trình bày văn bản bao gồm: Khổ giấy, kiểu trình bày, định lề trang, phông chữ, cỡ chữ, kiểu chữ, vị trí trình bày các thành phần thể thức, số trang văn bản. Kỹ thuật trình bày văn bản hành chính được thực hiện theo quy định tại Phụ lục I Nghị định này. Viết hoa trong văn bản hành chính được thực hiện theo quy định tại Phụ lục II Nghị định này. Chữ viết tắt tên loại văn bản hành chính được thực hiện theo quy định tại Phụ lục III Nghị định này.\r\n\r\nMục 2 SOẠN THẢO VÀ KÝ BAN HÀNH VĂN BẢN HÀNH CHÍNH\r\n\r\nĐiều 10. Soạn thảo văn bản\r\n1. Căn cứ chức năng, nhiệm vụ, quyền hạn và mục đích, nội dung của văn bản cần soạn thảo, người đứng đầu cơ quan, tổ chức hoặc người có thẩm quyền giao cho đơn vị hoặc cá nhân chủ trì soạn thảo văn bản.\r\n2. Đơn vị hoặc cá nhân được giao chủ trì soạn thảo văn bản thực hiện các công việc: Xác định tên loại, nội dung và độ mật, mức độ khẩn của văn bản cần soạn thảo; thu thập, xử lý thông tin có liên quan; soạn thảo văn bản đúng hình thức, thể thức và kỹ thuật trình bày.\r\nĐối với văn bản điện tử, cá nhân được giao nhiệm vụ soạn thảo văn bản ngoài việc thực hiện các nội dung nêu trên phải chuyển bản thảo văn bản, tài liệu kèm theo (nếu có) vào Hệ thống và cập nhật các thông tin cần thiết.\r\n3. Trường hợp cần sửa đổi, bổ sung bản thảo văn bản, người có thẩm quyền cho ý kiến vào bản thảo văn bản hoặc trên Hệ thống, chuyển lại bản thảo văn bản đến lãnh đạo đơn vị chủ trì soạn thảo văn bản để chuyển cho cá nhân được giao nhiệm vụ soạn thảo văn bản.\r\n4. Cá nhân được giao nhiệm vụ soạn thảo văn bản chịu trách nhiệm trước người đứng đầu đơn vị và trước pháp luật về bản thảo văn bản trong phạm vi chức trách, nhiệm vụ được giao.\r\nĐiều 11. Duyệt bản thảo văn bản\r\n1. Bản thảo văn bản phải do người có thẩm quyền ký văn bản duyệt.\r\n2. Trường hợp bản thảo văn bản đã được phê duyệt nhưng cần sửa chữa, bổ sung thì phải trình người có thẩm quyền ký xem xét, quyết định.\r\nĐiều 12. Kiểm tra văn bản trước khi ký ban hành\r\n1. Người đứng đầu đơn vị soạn thảo văn bản phải kiểm tra và chịu trách nhiệm trước người đứng đầu cơ quan, tổ chức và trước pháp luật về nội dung văn bản.\r\n2. Người được giao trách nhiệm kiểm tra thể thức, kỹ thuật trình bày văn bản phải kiểm tra và chịu trách nhiệm trước người đứng đầu cơ quan, tổ chức và trước pháp luật về thể thức, kỹ thuật trình bày văn bản.\r\nĐiều 13. Ký ban hành văn bản\r\n1. Cơ quan, tổ chức làm việc theo chế độ thủ trưởng\r\nNgười đứng đầu cơ quan, tổ chức có thẩm quyền ký tất cả văn bản do cơ quan, tổ chức ban hành; có thể giao cấp phó ký thay các văn bản thuộc lĩnh vực được phân công phụ trách và một số văn bản thuộc thẩm quyền của người đứng đầu. Trường hợp cấp phó được giao phụ trách, điều hành thì thực hiện ký như cấp phó ký thay cấp trưởng.\r\n2. Cơ quan, tổ chức làm việc theo chế độ tập thể\r\nNgười đứng đầu cơ quan, tổ chức thay mặt tập thể lãnh đạo ký các văn bản của cơ quan, tổ chức. Cấp phó của người đứng đầu cơ quan, tổ chức được thay mặt tập thể, ký thay người đứng đầu cơ quan, tổ chức những văn bản theo ủy quyền của người đứng đầu và những văn bản thuộc lĩnh vực được phân công phụ trách.\r\n3. Trong trường hợp đặc biệt, người đứng đầu cơ quan, tổ chức có thể ủy quyền cho người đứng đầu cơ quan, tổ chức, đơn vị thuộc cơ cấu tổ chức của mình ký thừa ủy quyền một số văn bản mà mình phải ký. Việc giao ký thừa ủy quyền phải được thực hiện bằng văn bản, giới hạn thời gian và nội dung được ủy quyền. Người được ký thừa ủy quyền không được ủy quyền lại cho người khác ký. Văn bản ký thừa ủy quyền được thực hiện theo thể thức và đóng dấu hoặc ký số của cơ quan, tổ chức ủy quyền.\r\n4. Người đứng đầu cơ quan, tổ chức có thể giao người đứng đầu đơn vị thuộc cơ quan, tổ chức ký thừa lệnh một số loại văn bản. Người được ký thừa lệnh được giao lại cho cấp phó ký thay. Việc giao ký thừa lệnh phải được quy định cụ thể trong quy chế làm việc hoặc quy chế công tác văn thư của cơ quan, tổ chức.\r\n5. Người ký văn bản phải chịu trách nhiệm trước pháp luật về văn bản do mình ký ban hành. Người đứng đầu cơ quan, tổ chức phải chịu trách nhiệm trước pháp luật về toàn bộ văn bản do cơ quan, tổ chức ban hành.\r\n6. Đối với văn bản giấy, khi ký văn bản dùng bút có mực màu xanh, không dùng các loại mực dễ phai.\r\n7. Đối với văn bản điện tử, người có thẩm quyền thực hiện ký số. Vị trí, hình ảnh chữ ký số theo quy định tại Phụ lục I Nghị định này.\r\nChương III QUẢN LÝ VĂN BẢN\r\nMục 1 QUẢN LÝ VĂN BẢN ĐI\r\nĐiều 14. Trình tự quản lý văn bản đi\r\n1. Cấp số, thời gian ban hành văn bản.\r\n2. Đăng ký văn bản đi.\r\n3. Nhân bản, đóng dấu cơ quan, tổ chức, dấu chỉ độ mật, mức độ khẩn, (đối với văn bản giấy); ký số của cơ quan, tổ chức (đối với văn bản điện tử).\r\n4. Phát hành và theo dõi việc chuyển phát văn bản đi.\r\n5. Lưu văn bản đi.\r\nĐiều 15. Cấp số, thời gian ban hành văn bản\r\n1. Số và thời gian ban hành văn bản được lấy theo thứ tự và trình tự thời gian ban hành văn bản của cơ quan, tổ chức trong năm (bắt đầu liên tiếp từ số 01 vào ngày 01 tháng 01 và kết thúc vào ngày 31 tháng 12 hàng năm). Số và ký hiệu văn bản của cơ quan, tổ chức là duy nhất trong một năm, thống nhất giữa văn bản giấy và văn bản điện tử.\r\na) Việc cấp số văn bản quy phạm pháp luật: Mỗi loại văn bản quy phạm pháp luật được cấp hệ thống số riêng.\r\nb) Việc cấp số văn bản chuyên ngành do người đứng đầu cơ quan quản lý ngành, lĩnh vực quy định.\r\nc) Việc cấp số văn bản hành chính do người đứng đầu cơ quan, tổ chức quy định.\r\n2. Đối với văn bản giấy, việc cấp số, thời gian ban hành được thực hiện sau khi có chữ ký của người có thẩm quyền, chậm nhất là trong ngày làm việc tiếp theo. Văn bản mật được cấp hệ thống số riêng.\r\n3. Đối với văn bản điện tử, việc cấp số, thời gian ban hành được thực hiện bằng chức năng của Hệ thống.\r\nĐiều 16. Đăng ký văn bản đi\r\n1. Việc đăng ký văn bản bảo đảm đầy đủ, chính xác các thông tin cần thiết của văn bản đi.\r\n2. Đăng ký văn bản\r\nVăn bản được đăng ký bằng sổ hoặc bằng Hệ thống.\r\na) Đăng ký văn bản bằng sổ\r\nVăn thư cơ quan đăng ký văn bản vào Sổ đăng ký văn bản đi. Mẫu Sổ đăng ký văn bản đi theo quy định tại Phụ lục IV Nghị định này.\r\nb) Đăng ký văn bản bằng Hệ thống\r\nVăn bản được đăng ký bằng Hệ thống phải được in ra giấy đầy đủ các trường thông tin theo mẫu Sổ đăng ký văn bản đi, đóng sổ để quản lý.\r\n3. Văn bản mật được đăng ký theo quy định của pháp luật về bảo vệ bí mật nhà nước.\r\nĐiều 17. Nhân bản, đóng dấu, ký số của cơ quan, tổ chức và dấu chỉ độ mật, mức độ khẩn\r\n1. Nhân bản, đóng dấu của cơ quan, tổ chức và dấu chỉ độ mật, mức độ khẩn đối với văn bản giấy\r\na) Văn bản đi được nhân bản theo đúng số lượng được xác định ở phần nơi nhận của văn bản.\r\nb) Việc đóng dấu cơ quan, tổ chức và dấu chỉ độ mật, mức độ khẩn, được thực hiện theo quy định tại Phụ lục I Nghị định này.\r\n2. Ký số của cơ quan, tổ chức đối với văn bản điện tử\r\nKý số của cơ quan, tổ chức được thực hiện theo quy định tại Phụ lục I Nghị định này.\r\nĐiều 18. Phát hành và theo dõi việc chuyển phát văn bản đi\r\n1. Văn bản đi phải hoàn thành thủ tục tại Văn thư cơ quan và phát hành trong ngày văn bản đó được ký, chậm nhất là trong ngày làm việc tiếp theo. Văn bản khẩn phải được phát hành và gửi ngay sau khi ký văn bản.\r\n2. Việc phát hành văn bản mật đi phải bảo đảm bí mật nội dung của văn bản theo quy định của pháp luật về bảo vệ bí mật nhà nước, đúng số lượng, thời gian và nơi nhận.\r\n3. Văn bản đã phát hành nhưng có sai sót về nội dung phải được sửa đổi, thay thế bằng văn bản có hình thức tương đương. Văn bản đã phát hành nhưng có sai sót về thể thức, kỹ thuật trình bày, thủ tục ban hành phải được đính chính bằng công văn của cơ quan, tổ chức ban hành văn bản.\r\n4. Thu hồi văn bản\r\na) Đối với văn bản giấy, trường hợp nhận được văn bản thông báo thu hồi, bên nhận có trách nhiệm gửi lại văn bản đã nhận.\r\nb) Đối với văn bản điện tử, trường hợp nhận được văn bản thông báo thu hồi, bên nhận hủy bỏ văn bản điện tử bị thu hồi trên Hệ thống, đồng thời thông báo qua Hệ thống để bên gửi biết.\r\n5. Phát hành văn bản giấy từ văn bản được ký số của người có thẩm quyền: Văn thư cơ quan thực hiện in văn bản đã được ký số của người có thẩm quyền ra giấy, đóng dấu của cơ quan, tổ chức để tạo bản chính văn bản giấy và phát hành văn bản.\r\n6. Trường hợp cần phát hành văn bản điện tử từ văn bản giấy: Văn thư cơ quan thực hiện theo quy định tại điểm c khoản 1 Điều 25 Nghị định này.\r\nĐiều 19. Lưu văn bản đi\r\n1. Lưu văn bản giấy\r\na) Bản gốc văn bản được lưu tại Văn thư cơ quan và phải được đóng dấu ngay sau khi phát hành, sắp xếp theo thứ tự đăng ký.\r\nb) Bản chính văn bản lưu tại hồ sơ công việc.\r\n2. Lưu văn bản điện tử\r\na) Bản gốc văn bản điện tử phải được lưu trên Hệ thống của cơ quan, tổ chức ban hành văn bản.\r\nb) Cơ quan, tổ chức có Hệ thống đáp ứng theo quy định tại Phụ lục VI Nghị định này và các quy định của pháp luật có liên quan thì sử dụng và lưu bản gốc văn bản điện tử trên Hệ thống thay cho văn bản giấy.\r\nc) Cơ quan, tổ chức có Hệ thống chưa đáp ứng theo quy định tại Phụ lục VI Nghị định này và các quy định của pháp luật có liên quan thì Văn thư cơ quan tạo bản chính văn bản giấy theo quy định tại khoản 5 Điều 18 Nghị định này để lưu tại Văn thư cơ quan và hồ sơ công việc.\r\nMục 2 QUẢN LÝ VĂN BẢN ĐẾN\r\nĐiều 20. Trình tự quản lý văn bản đến\r\n1. Tiếp nhận văn bản đến.\r\n2. Đăng ký văn bản đến.\r\n3. Trình, chuyển giao văn bản đến.\r\n4. Giải quyết và theo dõi, đôn đốc việc giải quyết văn bản đến.\r\nĐiều 21. Tiếp nhận văn bản đến\r\n1. Đối với văn bản giấy\r\na) Văn thư cơ quan kiểm tra số lượng, tình trạng bì, dấu niêm phong (nếu có), nơi gửi; đối chiếu số, ký hiệu ghi ngoài bì với số, ký hiệu của văn bản trong bì. Trường hợp phát hiện có sai sót hoặc dấu hiệu bất thường, Văn thư cơ quan báo ngay người có trách nhiệm giải quyết và thông báo cho nơi gửi văn bản.\r\nb) Tất cả văn bản giấy đến (bao gồm cả văn bản có dấu chỉ độ mật) gửi cơ quan, tổ chức thuộc diện đăng ký tại Văn thư cơ quan phải được bóc bì, đóng dấu “ĐẾN”. Đối với văn bản gửi đích danh cá nhân hoặc tổ chức đoàn thể trong cơ quan, tổ chức thì Văn thư cơ quan chuyển cho nơi nhận (không bóc bì). Những bì văn bản gửi đích danh cá nhân, nếu là văn bản liên quan đến công việc chung của cơ quan, tổ chức thì cá nhân nhận văn bản có trách nhiệm chuyển lại cho Văn thư cơ quan để đăng ký.\r\nc) Mẫu dấu “ĐẾN” được thực hiện theo quy định tại Phụ lục IV Nghị định này.\r\n2. Đối với văn bản điện tử\r\na) Văn thư cơ quan phải kiểm tra tính xác thực và toàn vẹn của văn bản điện tử và thực hiện tiếp nhận trên Hệ thống.\r\nb) Trường hợp văn bản điện tử không đáp ứng các quy định tại điểm a khoản này hoặc gửi sai nơi nhận thì cơ quan, tổ chức nhận văn bản phải trả lại cho cơ quan, tổ chức gửi văn bản trên Hệ thống. Trường hợp phát hiện có sai sót hoặc dấu hiệu bất thường thì Văn thư cơ quan báo ngay người có trách nhiệm giải quyết và thông báo cho nơi gửi văn bản.\r\nc) Cơ quan, tổ chức nhận văn bản có trách nhiệm thông báo ngay trong ngày cho cơ quan, tổ chức gửi về việc đã nhận văn bản bằng chức năng của Hệ thống.\r\nĐiều 22. Đăng ký văn bản đến\r\n1. Việc đăng ký văn bản đến phải bảo đảm đầy đủ, rõ ràng, chính xác các thông tin cần thiết theo mẫu sổ đăng ký văn bản đến hoặc theo thông tin đầu vào của dữ liệu quản lý văn bản đến. Những văn bản đến không được đăng ký tại Văn thư cơ quan thì đơn vị, cá nhân không có trách nhiệm giải quyết, trừ những loại văn bản đến được đăng ký riêng theo quy định của pháp luật.\r\n2. Số đến của văn bản được lấy liên tiếp theo thứ tự và trình tự thời gian tiếp nhận văn bản trong năm, thống nhất giữa văn bản giấy và văn bản điện tử.\r\n3. Đăng ký văn bản\r\nVăn bản được đăng ký bằng sổ hoặc bằng Hệ thống.\r\na) Đăng ký văn bản đến bằng sổ\r\nVăn thư cơ quan đăng ký văn bản vào sổ đăng ký văn bản đến. Mẫu sổ đăng ký văn bản đến theo quy định tại Phụ lục IV Nghị định này.\r\nb) Đăng ký văn bản đến bằng Hệ thống\r\nVăn thư cơ quan tiếp nhận văn bản và đăng ký vào Hệ thống. Trường hợp cần thiết, Văn thư cơ quan thực hiện số hóa văn bản đến theo quy định tại Phụ lục I Nghị định này. Văn thư cơ quan cập nhật vào Hệ thống các trường thông tin đầu vào của dữ liệu quản lý văn bản đến theo quy định tại Phụ lục VI Nghị định này. Văn bản đến được đăng ký vào Hệ thống phải được in ra giấy đầy đủ các trường thông tin theo mẫu Sổ đăng ký văn bản đến, ký nhận và đóng sổ để quản lý.\r\n4. Văn bản mật được đăng ký theo quy định của pháp luật về bảo vệ bí mật nhà nước.\r\nĐiều 23. Trình, chuyển giao văn bản đến\r\n1. Văn bản phải được Văn thư cơ quan trình trong ngày, chậm nhất là trong ngày làm việc tiếp theo đến người có thẩm quyền chỉ đạo giải quyết và chuyển giao cho đơn vị hoặc cá nhân được giao xử lý. Trường hợp đã xác định rõ đơn vị hoặc cá nhân được giao xử lý, Văn thư cơ quan chuyển văn bản đến đơn vị, cá nhân xử lý theo quy chế công tác văn thư của cơ quan, tổ chức. Văn bản đến có dấu chỉ các mức độ khẩn phải được trình và chuyển giao ngay sau khi nhận được. Việc chuyển giao văn bản phải bảo đảm chính xác và giữ bí mật nội dung văn bản.\r\n2. Căn cứ nội dung của văn bản đến; quy chế làm việc của cơ quan, tổ chức; chức năng, nhiệm vụ và kế hoạch công tác được giao cho đơn vị, cá nhân, người có thẩm quyền ghi ý kiến chỉ đạo giải quyết. Đối với văn bản liên quan đến nhiều đơn vị hoặc cá nhân thì xác định rõ đơn vị hoặc cá nhân chủ trì, phối hợp và thời hạn giải quyết.\r\n3. Trình, chuyển giao văn bản giấy: Ý kiến chỉ đạo giải quyết được ghi vào mục “Chuyển” trong dấu “ĐẾN” hoặc Phiếu giải quyết văn bản đến theo mẫu tại Phụ lục IV Nghị định này. Sau khi có ý kiến chỉ đạo giải quyết của người có thẩm quyền, văn bản đến được chuyển lại cho Văn thư cơ quan để đăng ký bổ sung thông tin, chuyển cho đơn vị hoặc cá nhân được giao giải quyết. Khi chuyển giao văn bản giấy đến cho đơn vị, cá nhân phải ký nhận văn bản.\r\n4. Trình, chuyển giao văn bản điện tử trên Hệ thống: Văn thư cơ quan trình văn bản điện tử đến người có thẩm quyền chỉ đạo giải quyết trên Hệ thống.\r\nNgười có thẩm quyền ghi ý kiến chỉ đạo giải quyết văn bản đến trên Hệ thống và cập nhật vào Hệ thống các thông tin: Đơn vị hoặc người nhận; ý kiến chỉ đạo, trạng thái xử lý văn bản; thời hạn giải quyết; chuyển văn bản cho đơn vị hoặc cá nhân được giao giải quyết. Trường hợp văn bản điện tử gửi kèm văn bản giấy thì Văn thư cơ quan thực hiện trình văn bản điện tử trên Hệ thống và chuyển văn bản giấy đến đơn vị hoặc cá nhân được người có thẩm quyền giao chủ trì giải quyết.\r\nĐiều 24. Giải quyết và theo dõi, đôn đốc việc giải quyết văn bản đến\r\n1. Người đứng đầu cơ quan, tổ chức có trách nhiệm chỉ đạo giải quyết kịp thời văn bản đến và giao người có trách nhiệm theo dõi, đôn đốc việc giải quyết văn bản đến.\r\n2. Khi nhận được văn bản đến, đơn vị hoặc cá nhân có trách nhiệm nghiên cứu, giải quyết văn bản đến theo thời hạn quy định tại quy chế làm việc của cơ quan, tổ chức. Những văn bản đến có dấu chỉ các mức độ khẩn phải được giải quyết ngay.\r\nMục 3 SAO VĂN BẢN\r\nĐiều 25. Các hình thức bản sao\r\n1. Sao y gồm: Sao y từ văn bản giấy sang văn bản giấy, sao y từ văn bản điện tử sang văn bản giấy, sao y từ văn bản giấy sang văn bản điện tử.\r\na) Sao y từ văn bản giấy sang văn bản giấy được thực hiện bằng việc chụp từ bản gốc hoặc bản chính văn bản giấy sang giấy.\r\nb) Sao y từ văn bản điện tử sang văn bản giấy được thực hiện bằng việc in từ bản gốc văn bản điện tử ra giấy.\r\nc) Sao y từ văn bản giấy sang văn bản điện tử được thực hiện bằng việc số hóa văn bản giấy và ký số của cơ quan, tổ chức.\r\n2. Sao lục\r\na) Sao lục gồm: Sao lục từ văn bản giấy sang văn bản giấy, sao lục từ văn bản giấy sang văn bản điện tử, sao lục từ văn bản điện tử sang văn bản giấy.\r\nb) Sao lục được thực hiện bằng việc in, chụp từ bản sao y.\r\n3. Trích sao\r\na) Trích sao gồm: Trích sao từ văn bản giấy sang văn bản giấy, trích sao từ văn bản giấy sang văn bản điện tử, trích sao từ văn bản điện tử sang văn bản điện tử, trích sao từ văn bản điện tử sang văn bản giấy.\r\nb) Bản trích sao được thực hiện bằng việc tạo lập lại đầy đủ thể thức, phần nội dung văn bản cần trích sao.\r\n4. Thể thức và kỹ thuật trình bày bản sao y, sao lục, trích sao được thực hiện theo quy định tại Phụ lục I Nghị định này.\r\nĐiều 26. Giá trị pháp lý của bản sao\r\nBản sao y, bản sao lục và bản trích sao được thực hiện theo đúng quy định tại Nghị định này có giá trị pháp lý như bản chính.\r\nĐiều 27. Thẩm quyền sao văn bản\r\n1. Người đứng đầu cơ quan, tổ chức quyết định việc sao văn bản do cơ quan, tổ chức ban hành, văn bản do các cơ quan, tổ chức khác gửi đến và quy định thẩm quyền ký các bản sao văn bản.\r\n2. Việc sao, chụp tài liệu, vật chứa bí mật nhà nước được thực hiện theo quy định của pháp luật về bảo vệ bí mật nhà nước.\r\nChương IV LẬP HỒ SƠ VÀ NỘP LƯU HỒ SƠ, TÀI LIỆU VÀO LƯU TRỮ CƠ QUAN\r\nĐiều 28. Lập Danh mục hồ sơ\r\nDanh mục hồ sơ do người đứng đầu cơ quan, tổ chức phê duyệt, được ban hành vào đầu năm và gửi các đơn vị, cá nhân liên quan làm căn cứ để lập hồ sơ. Mẫu Danh mục hồ sơ được thực hiện theo quy định tại Phụ lục V Nghị định này.\r\nĐiều 29. Lập hồ sơ\r\n1. Yêu cầu\r\na) Phản ánh đúng chức năng, nhiệm vụ của đơn vị, cơ quan, tổ chức.\r\nb) Các văn bản, tài liệu trong một hồ sơ phải có sự liên quan chặt chẽ với nhau và phản ánh đúng trình tự diễn biến của sự việc hoặc trình tự giải quyết công việc.\r\n2. Mở hồ sơ\r\na) Cá nhân được giao nhiệm vụ giải quyết công việc có trách nhiệm mở hồ sơ theo Danh mục hồ sơ hoặc theo kế hoạch công tác.\r\nb) Cập nhật những thông tin ban đầu về hồ sơ theo Danh mục hồ sơ đã ban hành.\r\nc) Trường hợp các hồ sơ không có trong Danh mục hồ sơ, cá nhân được giao nhiệm vụ giải quyết công việc tự xác định các thông tin: Tiêu đề hồ sơ, số và ký hiệu hồ sơ, thời hạn bảo quản hồ sơ, người lập hồ sơ và thời gian bắt đầu.\r\n3. Thu thập, cập nhật văn bản, tài liệu vào hồ sơ\r\nCá nhân được giao nhiệm vụ có trách nhiệm thu thập, cập nhật tất cả văn bản, tài liệu hình thành trong quá trình theo dõi, giải quyết công việc vào hồ sơ đã mở, bao gồm tài liệu phim, ảnh, ghi âm (nếu có) bảo đảm sự toàn vẹn, đầy đủ của hồ sơ, tránh bị thất lạc.\r\n4. Kết thúc hồ sơ\r\na) Hồ sơ được kết thúc khi công việc đã giải quyết xong.\r\nb) Người lập hồ sơ có trách nhiệm: Rà soát lại toàn bộ văn bản, tài liệu có trong hồ sơ; loại ra khỏi hồ sơ bản trùng, bản nháp; xác định lại thời hạn bảo quản của hồ sơ; chỉnh sửa tiêu đề, số và ký hiệu hồ sơ cho phù hợp; hoàn thiện, kết thúc hồ sơ.\r\nc) Đối với hồ sơ giấy: Người lập hồ sơ thực hiện đánh số tờ đối với hồ sơ có thời hạn bảo quản từ 05 năm trở lên và viết Mục lục văn bản đối với hồ sơ có thời hạn bảo quản vĩnh viễn; viết chứng từ kết thúc đối với tất cả hồ sơ.\r\nd) Đối với hồ sơ điện tử: Người lập hồ sơ có trách nhiệm cập nhật vào Hệ thống các thông tin còn thiếu. Việc biên mục văn bản trong hồ sơ được thực hiện bằng chức năng của Hệ thống.\r\nĐiều 30. Nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan\r\n1. Hồ sơ, tài liệu nộp lưu vào Lưu trữ cơ quan phải đủ thành phần, đúng thời hạn và thực hiện theo trình tự, thủ tục quy định.\r\n2. Thời hạn nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan\r\na) Đối với hồ sơ, tài liệu xây dựng cơ bản: Trong thời hạn 03 tháng kể từ ngày công trình được quyết toán.\r\nb) Đối với hồ sơ, tài liệu khác: Trong thời hạn 01 năm kể từ ngày công việc kết thúc.\r\n3. Thủ tục nộp lưu\r\na) Đối với hồ sơ giấy\r\nKhi nộp lưu tài liệu phải lập 02 bản “Mục lục hồ sơ, tài liệu nộp lưu” và 02 bản “Biên bản giao nhận hồ sơ, tài liệu” theo mẫu tại Phụ lục V Nghị định này. Đơn vị, cá nhân nộp lưu tài liệu và Lưu trữ cơ quan giữ mỗi loại 01 bản.\r\nb) Đối với hồ sơ điện tử\r\nCá nhân được giao nhiệm vụ giải quyết công việc và lập hồ sơ thực hiện nộp lưu hồ sơ điện tử vào Lưu trữ cơ quan trên Hệ thống.\r\nLưu trữ cơ quan có trách nhiệm kiểm tra, nhận hồ sơ theo Danh mục; liên kết chính xác dữ liệu đặc tả với hồ sơ; tiếp nhận và đưa hồ sơ về chế độ quản lý hồ sơ lưu trữ điện tử trên Hệ thống.\r\nĐiều 31. Trách nhiệm lập hồ sơ và nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan\r\n1. Người đứng đầu cơ quan, tổ chức trong phạm vi, nhiệm vụ, quyền hạn của mình có trách nhiệm quản lý văn bản, tài liệu của cơ quan, tổ chức; chỉ đạo, kiểm tra, hướng dẫn việc lập hồ sơ và nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan.\r\n2. Trách nhiệm của người đứng đầu bộ phận hành chính\r\na) Tham mưu cho người đứng đầu cơ quan, tổ chức trong việc chỉ đạo, kiểm tra, hướng dẫn việc lập hồ sơ và nộp lưu hồ sơ, tài liệu vào Lưu trữ cơ quan đối với cơ quan, tổ chức cấp dưới.\r\nb) Tổ chức thực hiện việc lập hồ sơ và nộp lưu hồ sơ, tài liệu vào Lưu trữ tại cơ quan, tổ chức.\r\n3. Trách nhiệm của đơn vị và cá nhân trong cơ quan, tổ chức\r\na) Người đứng đầu đơn vị trong cơ quan, tổ chức chịu trách nhiệm trước người đứng đầu cơ quan, tổ chức về việc lập hồ sơ, bảo quản và nộp lưu hồ sơ, tài liệu của đơn vị vào Lưu trữ cơ quan.\r\nb) Trong quá trình theo dõi, giải quyết công việc, mỗi cá nhân phải lập hồ sơ về công việc và chịu trách nhiệm về số lượng, thành phần, nội dung tài liệu trong hồ sơ; bảo đảm yêu cầu, chất lượng của hồ sơ theo quy định trước khi nộp lưu vào Lưu trữ cơ quan.\r\nc) Đơn vị và cá nhân trong cơ quan, tổ chức có trách nhiệm nộp lưu những hồ sơ, tài liệu được xác định thời hạn bảo quản từ 05 năm trở lên vào Lưu trữ cơ quan.\r\nd) Trường hợp đơn vị hoặc cá nhân có nhu cầu giữ lại hồ sơ, tài liệu đã đến hạn nộp lưu để phục vụ công việc thì phải được người đứng đầu cơ quan, tổ chức đồng ý bằng văn bản và phải lập Danh mục hồ sơ, tài liệu giữ lại gửi Lưu trữ cơ quan. Thời hạn giữ lại hồ sơ, tài liệu của đơn vị, cá nhân không quá 02 năm kể từ ngày đến hạn nộp lưu.\r\nđ) Cán bộ, công chức, viên chức và người lao động trong cơ quan, tổ chức trước khi nghỉ hưu, thôi việc, chuyển công tác, đi học tập dài ngày phải bàn giao toàn bộ hồ sơ, tài liệu hình thành trong quá trình công tác cho đơn vị, Lưu trữ cơ quan theo quy chế của cơ quan, tổ chức.\r\nChương V QUẢN LÝ, SỬ DỤNG CON DẤU VÀ THIẾT BỊ LƯU KHÓA BÍ MẬT TRONG CÔNG TÁC VĂN THƯ\r\nĐiều 32. Quản lý con dấu, thiết bị lưu khóa bí mật\r\n1. Người đứng đầu cơ quan, tổ chức có trách nhiệm giao cho Văn thư cơ quan quản lý, sử dụng con dấu, thiết bị lưu khóa bí mật của cơ quan, tổ chức theo quy định.\r\n2. Văn thư cơ quan có trách nhiệm\r\na) Bảo quản an toàn, sử dụng con dấu, thiết bị lưu khóa bí mật của cơ quan, tổ chức tại trụ sở cơ quan, tổ chức.\r\nb) Chỉ giao con dấu, thiết bị lưu khóa bí mật của cơ quan, tổ chức cho người khác khi được phép bằng văn bản của người có thẩm quyền. Việc bàn giao con dấu, thiết bị lưu khóa bí mật của cơ quan, tổ chức phải được lập biên bản.\r\nc) Phải trực tiếp đóng dấu, ký số vào văn bản do cơ quan, tổ chức ban hành và bản sao văn bản.\r\nd) Chỉ được đóng dấu, ký số của cơ quan, tổ chức vào văn bản đã có chữ ký của người có thẩm quyền và bản sao văn bản do cơ quan, tổ chức trực tiếp thực hiện.\r\n3. Cá nhân có trách nhiệm tự bảo quản an toàn thiết bị lưu khóa bí mật và khóa bí mật.\r\nĐiều 33. Sử dụng con dấu, thiết bị lưu khóa bí mật\r\n1. Sử dụng con dấu\r\na) Dấu đóng phải rõ ràng, ngay ngắn, đúng chiều và dùng đúng mực dấu màu đỏ theo quy định.\r\nb) Khi đóng dấu lên chữ ký, dấu đóng phải trùm lên khoảng 1/3 chữ ký về phía bên trái.\r\nc) Các văn bản ban hành kèm theo văn bản chính hoặc phụ lục: Dấu được đóng lên trang đầu, trùm một phần tên cơ quan, tổ chức hoặc tiêu đề phụ lục.\r\nd) Việc đóng dấu treo, dấu giáp lai, đóng dấu nổi trên văn bản giấy do người đứng đầu cơ quan, tổ chức quy định.\r\nđ) Dấu giáp lai được đóng vào khoảng giữa mép phải của văn bản hoặc phụ lục văn bản, trùm lên một phần các tờ giấy; mỗi dấu đóng tối đa 05 tờ văn bản.\r\n2. Sử dụng thiết bị lưu khóa bí mật\r\nThiết bị lưu khóa bí mật của cơ quan, tổ chức được sử dụng để ký số các văn bản điện tử do cơ quan, tổ chức ban hành và bản sao từ văn bản giấy sang văn bản điện tử.\r\nChương VI QUẢN LÝ NHÀ NƯỚC VỀ CÔNG TÁC VĂN THƯ\r\nĐiều 34. Nội dung quản lý nhà nước về công tác văn thư\r\n1. Xây dựng, ban hành và chỉ đạo, hướng dẫn thực hiện các văn bản quy phạm pháp luật về công tác văn thư.\r\n2. Quản lý thống nhất về nghiệp vụ công tác văn thư.\r\n3. Quản lý nghiên cứu khoa học, ứng dụng khoa học và công nghệ trong công tác văn thư.\r\n4. Quản lý đào tạo, bồi dưỡng người làm công tác văn thư; quản lý công tác thi đua, khen thưởng trong công tác văn thư.\r\n5. Thanh tra, kiểm tra, giải quyết khiếu nại, tố cáo và xử lý vi phạm pháp luật về công tác văn thư.\r\n6. Hợp tác quốc tế trong công tác văn thư.\r\n7. Sơ kết, tổng kết công tác văn thư.\r\nĐiều 35. Trách nhiệm quản lý công tác văn thư\r\n1. Bộ Nội vụ chịu trách nhiệm trước Chính phủ thực hiện quản lý nhà nước về công tác văn thư.\r\n2. Các bộ, cơ quan ngang bộ, cơ quan thuộc Chính phủ, Ủy ban nhân dân các cấp, doanh nghiệp nhà nước trong phạm vi nhiệm vụ, quyền hạn của mình có trách nhiệm:\r\na) Căn cứ quy định của pháp luật, ban hành và hướng dẫn thực hiện các quy định về công tác văn thư.\r\nb) Kiểm tra việc thực hiện các quy định về công tác văn thư đối với các cơ quan, tổ chức thuộc phạm vi quản lý; giải quyết khiếu nại, tố cáo và xử lý vi phạm pháp luật về công tác văn thư theo thẩm quyền.\r\nc) Tổ chức, chỉ đạo việc nghiên cứu, ứng dụng khoa học và công nghệ trong công tác văn thư.\r\nd) Bố trí kinh phí để hiện đại hóa phương tiện, hạ tầng kỹ thuật phục vụ công tác văn thư, quản lý và vận hành hiệu quả Hệ thống quản lý tài liệu điện tử.\r\nđ) Bố trí nhân sự, vị trí, diện tích, phương tiện làm việc phù hợp, bảo đảm giữ gìn bí mật nhà nước, bảo quản an toàn con dấu, thiết bị lưu khóa bí mật của cơ quan, tổ chức.\r\ne) Tổ chức đào tạo, bồi dưỡng người làm công tác văn thư; quản lý công tác thi đua, khen thưởng trong công tác văn thư.\r\ng) Sơ kết, tổng kết về công tác văn thư trong phạm vi ngành, lĩnh vực và địa phương.\r\nĐiều 36. Kinh phí cho công tác văn thư\r\n1. Các cơ quan, tổ chức có trách nhiệm bố trí kinh phí cho công tác văn thư trong dự toán ngân sách nhà nước hàng năm. Đối với doanh nghiệp nhà nước việc bố trí kinh phí được thực hiện theo quy định hiện hành.\r\n2. Kinh phí cho công tác văn thư được sử dụng vào các công việc\r\na) Mua sắm, nâng cấp hệ thống, hạ tầng kỹ thuật, trang thiết bị, vật tư tiêu hao phục vụ công tác văn thư.\r\nb) Bảo đảm thông tin liên lạc, chuyển phát văn bản, số hóa văn bản.\r\nc) Nghiên cứu, ứng dụng khoa học và chuyển giao công nghệ trong công tác văn thư.\r\nd) Các hoạt động khác phục vụ công tác văn thư.\r\nChương VII ĐIỀU KHOẢN THI HÀNH\r\nĐiều 37. Hiệu lực thi hành\r\nNghị định này có hiệu lực thi hành kể từ ngày ký. Nghị định số 110/2004/NĐ-CP ngày 08 tháng 4 năm 2004 của Chính phủ về công tác văn thư và Nghị định số 09/2010/NĐ-CP ngày 08 tháng 02 năm 2010 của Chính phủ sửa đổi, bổ sung một số điều của Nghị định số 110/2004/NĐ-CP ngày 08 tháng 4 năm 2004 của Chính phủ về công tác văn thư hết hiệu lực từ ngày Nghị định này có hiệu lực pháp luật.\r\nĐiều 38. Trách nhiệm thi hành\r\n1. Bộ trưởng Bộ Nội vụ có trách nhiệm triển khai thực hiện và kiểm tra việc thi hành Nghị định này.\r\n2. Các Bộ trưởng, Thủ trưởng cơ quan ngang bộ, Thủ trưởng cơ quan thuộc Chính phủ, Chủ tịch Ủy ban nhân dân tỉnh, thành phố trực thuộc trung ương, người đứng đầu các doanh nghiệp nhà nước và các tổ chức, cá nhân có liên quan chịu trách nhiệm thi hành Nghị định này./.\r\n\r\nIII. MẪU MỤC LỤC HỒ SƠ, TÀI LIỆU NỘP LƯU\r\n\r\nTÊN CQ, TC CHỦ QUẢN\r\n\r\nTÊN ĐƠN VỊ1\r\n\r\n________\r\n\r\n \r\n\r\nCỘNG HÒA XÃ HỘI CHỦ NGHĨA VIỆT NAM\r\n\r\nĐộc lập - Tự do - Hạnh phúc\r\n\r\n________________________', '30', 'VN', 'ND', NULL, 'Hà Nội', NULL, NULL, 'HS01', 'Đã biết', 'NĐ-CP');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `languages`
--

CREATE TABLE `languages` (
  `LanId` varchar(10) NOT NULL,
  `Language` varchar(30) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `languages`
--

INSERT INTO `languages` (`LanId`, `Language`) VALUES
('EN', 'Tiếng Anh'),
('VN', 'Tiếng Việt');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `organization`
--

CREATE TABLE `organization` (
  `OranId` varchar(13) NOT NULL,
  `OranName` varchar(100) DEFAULT NULL,
  `PosId` varchar(20) DEFAULT NULL,
  `ToPlaces` text DEFAULT NULL,
  `ID` varchar(30) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `organization`
--

INSERT INTO `organization` (`OranId`, `OranName`, `PosId`, `ToPlaces`, `ID`) VALUES
('CQ01', 'Chính phủ', NULL, NULL, 'ND01');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `positions`
--

CREATE TABLE `positions` (
  `PosId` varchar(20) NOT NULL,
  `SingerInfo` varchar(255) DEFAULT NULL,
  `Position` varchar(100) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `positions`
--

INSERT INTO `positions` (`PosId`, `SingerInfo`, `Position`) VALUES
('CK01', 'Nguyễn Xuân Phúc', 'Chủ tịch nước');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `signatures`
--

CREATE TABLE `signatures` (
  `SignId` varchar(30) DEFAULT NULL,
  `SignDate` date DEFAULT NULL,
  `FileCode` varchar(50) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `signatures`
--

INSERT INTO `signatures` (`SignId`, `SignDate`, `FileCode`) VALUES
('ND01', '2020-03-05', 'HS01');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `summary`
--

CREATE TABLE `summary` (
  `InfoId` varchar(50) NOT NULL,
  `content` text NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

--
-- Đang đổ dữ liệu cho bảng `summary`
--

INSERT INTO `summary` (`InfoId`, `content`) VALUES
('TT01', 'TÓM TẮT VĂN BẢN\r\nHướng dẫn mới nhất về kỹ thuật trình bày văn bản hành chính\r\n\r\nChính phủ đã ban hành Nghị định 30/2020/NĐ-CP về công tác văn thư ngày 05/3/2020 với một số nội dung đáng chú ý như sau: \r\n- Văn bản hành chính được trình bày trên khổ giấy A4 (210 mm X 297 mm).\r\n\r\n- Phông chữ tiếng Việt Times New Roman, bộ mã ký tự Unicode theo Tiêu chuẩn Việt Nam TCVN 6909:2001, màu đen.\r\n\r\n- Số trang văn bản được đánh từ số 1, bằng chữ số Ả Rập, cỡ chữ 13 đến 14, kiểu chữ đứng, được đặt canh giữa theo chiều ngang trong phần lề trên của văn bản, không hiển thị số trang thứ nhất.\r\n\r\n- Thời gian ban hành văn bản phải được viết đầy đủ; các số thể hiện ngày, tháng, năm dùng chữ số Ả Rập; đối với những số thể hiện ngày nhỏ hơn 10 và tháng 1, 2 phải ghi thêm số 0 phía trước.\r\n\r\n- Khi viện dẫn lần đầu văn bản có liên quan, phải ghi đầy đủ tên loại, số, ký hiệu của văn bản, thời gian ban hành văn bản, tên cơ quan, tổ chức ban hành văn bản và trích yếu nội dung văn bản (đối với Luật và Pháp lệnh chỉ ghi tên loại và tên của Luật, Pháp lệnh); trong các lần viện dẫn tiếp theo, chỉ ghi tên loại và số, ký hiệu của văn bản đó…\r\n\r\n- Trước họ tên của người ký, không ghi học hàm, học vị và các danh hiệu danh dự khác. Việc ghi thêm quân hàm, học hàm, học vị trước họ tên người ký đối với văn bản của các đơn vị vũ trang nhân dân, các tổ chức sự nghiệp giáo dục, y tế, khoa học do người đứng đầu cơ quan quản lý ngành, lĩnh vực quy định.\r\n\r\nNghị định này có hiệu lực từ ngày 05/3/2020.');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `type_documents`
--

CREATE TABLE `type_documents` (
  `TypeId` varchar(10) NOT NULL,
  `TypeName` varchar(100) DEFAULT NULL,
  `OranId` varchar(13) DEFAULT NULL,
  `Priority` decimal(10,2) DEFAULT NULL,
  `IssuedAmount` decimal(10,2) DEFAULT NULL,
  `DueDate` date DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `type_documents`
--

INSERT INTO `type_documents` (`TypeId`, `TypeName`, `OranId`, `Priority`, `IssuedAmount`, `DueDate`) VALUES
('ND', 'Nghị định', 'CQ01', NULL, NULL, NULL);

--
-- Chỉ mục cho các bảng đã đổ
--

--
-- Chỉ mục cho bảng `account`
--
ALTER TABLE `account`
  ADD PRIMARY KEY (`ID`);

--
-- Chỉ mục cho bảng `files`
--
ALTER TABLE `files`
  ADD PRIMARY KEY (`FileCode`),
  ADD KEY `OranId` (`OranId`),
  ADD KEY `TypeId` (`TypeId`);

--
-- Chỉ mục cho bảng `infomation_documents_arrival`
--
ALTER TABLE `infomation_documents_arrival`
  ADD KEY `InfoId` (`InfoId`);

--
-- Chỉ mục cho bảng `infomation_documents_out`
--
ALTER TABLE `infomation_documents_out`
  ADD PRIMARY KEY (`InfoId`),
  ADD KEY `TypeId` (`TypeId`),
  ADD KEY `LanId` (`LanId`),
  ADD KEY `FileCode` (`FileCode`);

--
-- Chỉ mục cho bảng `languages`
--
ALTER TABLE `languages`
  ADD PRIMARY KEY (`LanId`);

--
-- Chỉ mục cho bảng `organization`
--
ALTER TABLE `organization`
  ADD PRIMARY KEY (`OranId`),
  ADD KEY `ID` (`ID`),
  ADD KEY `PosId` (`PosId`);

--
-- Chỉ mục cho bảng `positions`
--
ALTER TABLE `positions`
  ADD PRIMARY KEY (`PosId`);

--
-- Chỉ mục cho bảng `signatures`
--
ALTER TABLE `signatures`
  ADD KEY `SignId` (`SignId`),
  ADD KEY `FileCode` (`FileCode`);

--
-- Chỉ mục cho bảng `type_documents`
--
ALTER TABLE `type_documents`
  ADD PRIMARY KEY (`TypeId`),
  ADD KEY `OranId` (`OranId`);

--
-- Các ràng buộc cho các bảng đã đổ
--

--
-- Các ràng buộc cho bảng `files`
--
ALTER TABLE `files`
  ADD CONSTRAINT `files_ibfk_1` FOREIGN KEY (`OranId`) REFERENCES `organization` (`OranId`),
  ADD CONSTRAINT `files_ibfk_2` FOREIGN KEY (`TypeId`) REFERENCES `type_documents` (`TypeId`);

--
-- Các ràng buộc cho bảng `infomation_documents_arrival`
--
ALTER TABLE `infomation_documents_arrival`
  ADD CONSTRAINT `infomation_documents_arrival_ibfk_1` FOREIGN KEY (`InfoId`) REFERENCES `infomation_documents_out` (`InfoId`);

--
-- Các ràng buộc cho bảng `infomation_documents_out`
--
ALTER TABLE `infomation_documents_out`
  ADD CONSTRAINT `infomation_documents_out_ibfk_1` FOREIGN KEY (`TypeId`) REFERENCES `type_documents` (`TypeId`),
  ADD CONSTRAINT `infomation_documents_out_ibfk_2` FOREIGN KEY (`LanId`) REFERENCES `languages` (`LanId`),
  ADD CONSTRAINT `infomation_documents_out_ibfk_3` FOREIGN KEY (`FileCode`) REFERENCES `files` (`FileCode`);

--
-- Các ràng buộc cho bảng `organization`
--
ALTER TABLE `organization`
  ADD CONSTRAINT `organization_ibfk_1` FOREIGN KEY (`ID`) REFERENCES `account` (`ID`),
  ADD CONSTRAINT `organization_ibfk_2` FOREIGN KEY (`PosId`) REFERENCES `positions` (`PosId`);

--
-- Các ràng buộc cho bảng `signatures`
--
ALTER TABLE `signatures`
  ADD CONSTRAINT `signatures_ibfk_1` FOREIGN KEY (`SignId`) REFERENCES `account` (`ID`),
  ADD CONSTRAINT `signatures_ibfk_2` FOREIGN KEY (`FileCode`) REFERENCES `files` (`FileCode`);

--
-- Các ràng buộc cho bảng `type_documents`
--
ALTER TABLE `type_documents`
  ADD CONSTRAINT `type_documents_ibfk_1` FOREIGN KEY (`OranId`) REFERENCES `organization` (`OranId`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
