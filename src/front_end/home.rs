
pub fn homemain() -> String {
    format!(
        r#"
        <!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nghị định 30/2020/NĐ-CP về công tác văn thư</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" rel="stylesheet" />
    <link rel="stylesheet" href="static/nghidinh.css">
    <script src="handle/handlepage.js"></script>
    <script src="handle/main.js"></script>
</head>
<body>
    <header class="bg-white shadow">
        <div class="container mx-auto flex justify-between items-center py-4 px-6">
            <div class="flex items-center">
                <img alt="LuatVietnam logo" class="h-10" height="300" width="50" src="https://th.bing.com/th/id/OIP.-_GKfmytz61oq-HnVMf0NQHaHa?rs=1&pid=ImgDetMain" />
                <span class="ml-2 text-sm text-gray-600">Tiện ích văn bản luật</span>
            </div>
            <div class="flex items-center space-x-4">
                <button class="text-gray-600"></button>
                <span class="text-red-600"></span>
                <button class="text-gray-600"></button>
                <button class="text-gray-600"></button>
                <button class="text-gray-600">Đăng ký / Đăng nhập</button>
            </div>
        </div>
    </header>
    <nav class="bg-gray-50 shadow">
        <div class="container mx-auto flex justify-between items-center py-2 px-6">
            <div class="flex space-x-4">
                <a class="text-gray-600 hover:text-gray-800" href=""><i class="fas fa-home"></i>Văn bản mới</a>
                <a class="text-gray-600 hover:text-gray-800" href="">Tra cứu văn bản</a>
                <a class="text-gray-600 hover:text-gray-800" href=""> Dự thảo </a>
                <a class="text-gray-600 hover:text-gray-800" href="">Tin văn bản mới</a>
                <a class="text-gray-600 hover:text-gray-800" href="">Tin pháp luật</a>
                <a class="text-gray-600 hover:text-gray-800" href="">Bản tin luật</a>
                <a class="text-gray-600 hover:text-gray-800" href="">Luật sư tư vấn</a>
                <a class="text-gray-600 hover:text-gray-800" href="">Pháp lý doanh nghiệp</a>
                <a class="text-gray-600 hover:text-gray-800" href=""> Tiện ích </a>
            </div>
            <button class="text-gray-600"><i class="fas fa-bars"> </i></button>
        </div>
    </nav>
    <main class="container mx-auto py-6 px-6">
        <div class="flex items-center space-x-2 text-gray-600 text-sm" >
            <a class="hover:underline" href=""> Văn bản mới </a>
            <span> / </span>
           
        </div>
        <div class="flex">
            <div class="mt-6">
                <div class="flex items-center">
                    <input class="border border-gray-300 rounded-l px-4 py-2" style="width: 600px;" placeholder="Tìm kiếm văn bản..." type="text"/>
                    <button class="bg-red-600 text-white px-4 py-2 rounded-r">Tìm kiếm</button>
                    <button class="ml-2 text-gray-600"><i class="fas fa-cogs"> </i>Tìm năng cao</button>
                </div>
                <h1 class="text-2xl font-bold mt-2">Nghị định 30/2020/NĐ-CP về công tác văn thư</h1>
                <div class="contain_button">
                    <div class="flex space-x-4 mt-4">
                        
                    </div>
                </div>

                <div class="change page active_page" id="page_first">
                    <div class="bg-gray p-1 mt-4 pl-4 shadow rounded w-[850px] h-[70px]">
                        <div class="flex space-x-2 mt-2">
                           
                        </div>
                    </div>
                    <div class="bg-white p-4 mt-4 shadow rounded w-[850px] h-[430px] overflow-y-auto">
                        <div style="height: 25px; display: flex; align-items: center; justify-content: space-between;">
                            <div style="display: flex; align-items: center;">
                                <p style="font-size: 15px; margin-right: 10px; position: relative; top: -5px;">Tình trạng hiệu lực:</p>
                                <strong><p style="font-size: 15px; top: -5px; margin-right: 10px; position: relative;" id="validitystatus"></p></strong>
                            </div>
                            <p style="font-size: 15px; top: -5px; margin-right: 10px; position: relative;">Ghi chú</p>
                        </div>
                        <hr>
                        <br>
                        <div class="header-decree">
                            
                        </div>
                        <div class="content-section">
                           
                        </div>
                    </div>
                </div>
                
            </div>
            
           
        </div>
    </main>
</body>
</html>
        "#
    )
}