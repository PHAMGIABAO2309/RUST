use crate::front_end::content::document_content;
pub fn home( title: &str,  content: &str) -> String {
    let document_html = document_content( title,  content);
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
                {document_html}
            </div>
        </div>
    </main>
</body>
</html>"#,
    )
}
