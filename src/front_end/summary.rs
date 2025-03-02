pub fn summary() -> String {
    format! (
        r#"
        <div class="change page hidden" id="page_two">
            <div class="bg-gray p-1 mt-4 pl-4 shadow rounded w-[850px] h-[70px]">
                <div class="flex space-x-2 mt-2">
                    <button class="bg-red-600 text-white px-4 py-2 rounded     item">VB Song ngữ</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item">Tải VB</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item"> Lưu</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item"> Theo dõi VB</button>
                    <button class="bg-gray-200 text-gray-800 px-4 py-2 rounded item"> Báo lỗi</button>
                </div>
            </div>
            <div class=" bg-white p-4 mt-4 shadow rounded w-[850px] h-[430px] overflow-y-auto no-scrollbar " >
                <div class="header-decree">
                    <div>
                        <p>CHÍNH PHỦ</p>
                        <p>______</p>
                        Số: <span id="sovanban"></span>/<span id="namhinhthanh"></span>/NĐ-CP
                    </div>
                    <div>
                        <p>CỘNG HÒA XÃ HỘI CHỦ NGHĨA VIỆT NAM</p>
                        <p class="header-title">Độc lập - Tự do - Hạnh phúc</p>
                        <span class="underline"></span>
                        <span id="noinhan"></span>, ngày 05 tháng 3 năm <span id="namhinhthanh"></span>
                    </div>
                </div>
                <div class="content-section">
                    <p id="content">Vui lòng chờ...</p>
                </div>
            </div>
        </div>
        "#
    )
}