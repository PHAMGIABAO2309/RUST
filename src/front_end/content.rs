pub fn document_content(poem_content: &str) -> String {
    format!(
        r#"<div class="bg-white p-4 mt-4 shadow w-[1000px] h-[430px] overflow-y-auto no-scrollbar border border-gray-300">
      <h2 class="text-xl font-bold">CHÍNH PHỦ</h2>
      <p class="mt-2">Số: 30/2020/NĐ-CP</p>
      <p class="mt-2">Hà Nội, ngày 05 tháng 3 năm 2020</p>
      <p>{}</p>
    </div>"#,
        poem_content
    )
}
