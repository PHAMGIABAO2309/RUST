use crate::front_end::content::document_content;
pub fn home( title: &str,  content: &str) -> String {
    let document_html = document_content( title, content);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta content="width=device-width, initial-scale=1.0" name="viewport" />
    <title>Document</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" rel="stylesheet" />
    <link rel="stylesheet" href="/static/index.css" />
</head>
<body class="bg-gray-100">
    
    <main class="container mx-auto py-6 px-6">
        <div>
            <div class="mt-4 bg-white p-4 shadow rounded">
                <div class="bg-white p-4 mt-4 shadow w-[1000px] h-[430px] overflow-y-auto no-scrollbar ">
                    <div class="header-decree">
                        <div>
                            <p>CHÍNH PHỦ</p>
                            <p>______</p>
                            <p class="doc-number">Số: 30/2020/NĐ-CP</p>
                        </div>
                        <div>
                            <p>CỘNG HÒA XÃ HỘI CHỦ NGHĨA VIỆT NAM</p>
                            <p class="header-title">Độc lập - Tự do - Hạnh phúc</p>
                            <span class="underline"></span>
                            <p class="date">Hà Nội, ngày 05 tháng 3 năm 2020</p>
                        </div>
                    </div>
                    {document_html}
                </div>
            </div>
        </div>
    </main>
</body>
</html>"#,
    )
}
