pub fn home(poem_content: &str) -> String {
  format!(
      r#"<!DOCTYPE html>
      <html lang="en">
      <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <title>Tài liệu pháp luật</title>
          <script src="https://cdn.tailwindcss.com"></script>
          <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css">
      </head>
      <body class="bg-gray-100 text-gray-900">
          <header class="bg-white shadow">
              <div class="container flex justify-between items-center py-4">
                  <div class="flex items-center">
                      <img src="https://storage.googleapis.com/a1aa/image/1id_n8_P-1fCXoyPjLLKZzD48YWK4FDjAV34G1wRj_k.jpg" class="h-10">
                      <span class="ml-2 text-sm text-gray-600">Tiện ích văn bản luật</span>
                  </div>
                  <div class="flex items-center space-x-4">
                      <button class="text-gray-600 hover:text-black transition">Danh mục</button>
                      <span class="text-red-600 font-semibold">Tổng đài trực tuyến 19006192</span>
                      <button class="text-gray-600 hover:text-black transition">Gói dịch vụ & Giá</button>
                      <button class="text-gray-600 hover:text-black transition">Tiếng Anh</button>
                      <button class="bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition">
                          Đăng ký / Đăng nhập
                      </button>
                  </div>
              </div>
          </header>

          <main class="container py-6">
              <h1 class="text-2xl font-bold mt-6">Nghị định 30/2020/NĐ-CP về công tác văn thư</h1>
              <div class="bg-white p-4 mt-4 shadow rounded-md">
                  <h2 class="text-xl font-bold">CHÍNH PHỦ</h2>
                  <p class="mt-2">Số: 30/2020/NĐ-CP</p>
                  <p class="mt-2">Hà Nội, ngày 05 tháng 3 năm 2020</p>
                  <img src="https://storage.googleapis.com/a1aa/image/heGjcg_7YbPP9Mcf-3wHvkQQ2k8FT_rDkc1k3NmthGk.jpg" class="mt-4">
              </div>

              <div class="mt-6 p-4 bg-gray-100 rounded-md shadow">
                  <h3 class="font-bold">Nội dung văn bản:</h3>
                  <p class="mt-2">{}</p>
              </div>
          </main>
      </body>
      </html>"#,
      poem_content
  )
}
