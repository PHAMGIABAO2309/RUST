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
                <div>
                    <h1 class="text-xl font-bold mb-2">THUỘC TÍNH NGHỊ ĐỊNH 30/2020/NĐ-CP</h1>
                    <p class="mb-4">Nghị định 30/2020/NĐ-CP của Chính phủ về công tác văn thư</p>
                    <div class="overflow-x-auto">
                        <table class="min-w-full bg-white border border-gray-300">
                            <tbody>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Cơ quan ban hành:</td>
                                    <td class="px-4 py-2 border-r" id="chinhphu"></td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Số công báo:</td>
                                    <td class="px-4 py-2 text-red-500">Đã biết <i class="fas fa-info-circle"></i></td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Số hiệu:</td>
                                    <td class="px-4 py-2 border-r" id="sohieuuu"></td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Ngày đăng công báo:</td>
                                    <td class="px-4 py-2 text-red-500">Đã biết <i class="fas fa-info-circle"></i></td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Loại văn bản:</td>
                                    <td class="px-4 py-2 border-r" id="loaivanban"></td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Người ký:</td>
                                    <td class="px-4 py-2" id="nguoikyy"></td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Ngày ban hành:</td>
                                    <td class="px-4 py-2 border-r" id="ngaybanhanhh"></td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Ngày hết hiệu lực:</td>
                                    <td class="px-4 py-2 text-gray-500" id="ngayhethieuluc"></td>
                                </tr>
                                <tr class="border-b">
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Áp dụng:</td>
                                    <td class="px-4 py-2 text-red-500 border-r">Đã biết <i class="fas fa-info-circle"></i></td>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Tình trạng hiệu lực:</td>
                                    <td class="px-4 py-2 text-red-500">Đã biết <i class="fas fa-info-circle"></i></td>
                                </tr>
                                <tr>
                                    <td class="px-4 py-2 font-semibold border-r bg-gray-100">Lĩnh vực:</td>
                                    <td class="px-4 py-2 border-r">Hành chính</td>
                                    <td class="px-4 py-2 border-r bg-gray-100"></td>
                                    <td class="px-4 py-2"></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>

                <div>
                    <p id="tomtat"></p>
                </div>
            </div>
        </div>
        "#
    )
}