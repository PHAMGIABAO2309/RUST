pub fn side_panel() -> String {
    format!(
        r#"
        <div class="max-w-sm p-6 ">
            <!-- Reference Section -->
            <div class="bg-white shadow-md rounded-lg mb-4">
                <div class="bg-gray-200 p-2 rounded-t-lg">
                    <h2 class="text-gray-700 font-semibold">THAM CHIẾU ĐẾN NỘI DUNG</h2>
                </div>
                <div class="p-2">
                    <div class="flex items-center mb-2">
                        <i class="fas fa-plus-circle text-black mr-2"></i>
                        <span class="text-black">Câu hỏi thường gặp</span>
                    </div>
                    <div class="flex items-center">
                        <i class="fas fa-plus-circle text-black mr-2"></i>
                        <span class="text-black">Văn bản cùng lĩnh vực</span>
                    </div>
                </div>
            </div>
            <!-- Document Category Section -->
            <div class="bg-white shadow-md rounded-lg">
                <div class="bg-gray-200 p-2 rounded-t-lg">
                    <h2 class="text-gray-700 font-semibold">DANH MỤC CÁC BẢN MIX</h2>
                </div>
                <div class="p-2">
                    <h3 class="text-black font-semibold">Timeline</h3>
                    <p class="text-gray-500 text-sm mb-2">Nội dung MIX tại thời điểm có VB mới ban hành tác động đến văn bản đang xem</p>
                    <h4 class="text-teal-600 font-semibold mb-2">Văn bản bị tác động thay đổi nội dung</h4>
                    
                    <!-- Document Item 1 -->
                    <div class="mb-4">
                        <h5 class="text-yellow-700 font-semibold">Nghị định 110/2004/NĐ-CP của Chính phủ về công tác văn thư</h5>
                        <p class="text-gray-500 text-sm">Ban hành: 08/04/2004</p>
                        <p class="text-gray-500 text-sm mb-2">Hiệu lực: Đã biết</p>
                        <input type="checkbox" class="form-checkbox">
                    </div>
                    
                    <!-- Document Item 2 -->
                    <div>
                        <h5 class="text-yellow-700 font-semibold">Nghị định 09/2010/NĐ-CP sửa đổi Nghị định về công tác văn thư</h5>
                        <p class="text-gray-500 text-sm">Ban hành: 08/02/2010</p>
                        <p class="text-gray-500 text-sm mb-2" id="validitystatus"> </p>
                        <input type="checkbox" class="form-checkbox">
                    </div>
                </div>
            </div>
        </div>
        "#
    )
}
