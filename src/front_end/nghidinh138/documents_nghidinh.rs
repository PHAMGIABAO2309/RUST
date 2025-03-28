pub fn handle_documents_nghidinh_js() -> String {
    r#"
        function handleDocumentsNghidinh(documents_nghidinh) {
            if(documents_nghidinh.length === 0 ) return;
            const firstDocumentsNghidinh = documents_nghidinh[0];
            if(firstDocumentsNghidinh.title){
                let tieudeEl = document.getElementById("title_nghidinh");
                if (tieudeEl) tieudeEl.innerText = firstDocumentsNghidinh.title;
            }
            if(firstDocumentsNghidinh.codenumber){
                let sovanbanEl = document.getElementById("sovanban_nghidinh");
                if (sovanbanEl) sovanbanEl.innerText = firstDocumentsNghidinh.codenumber;
            }
            if(firstDocumentsNghidinh.receives){
                let noinhanEl = document.getElementById("noinhan_nghidinh");
                if (noinhanEl) noinhanEl.innerText = firstDocumentsNghidinh.receives;
            }
            if (firstDocumentsNghidinh.filecatalog) {
                let namhinhthanhElements = document.querySelectorAll("[id='namhinhthanh_nghidinh']");
                namhinhthanhElements.forEach(el => {
                    el.innerText = firstDocumentsNghidinh.filecatalog;
                });
            }
            if(firstDocumentsNghidinh.subject){
                let noidungEl = document.getElementById("content_nghidinh");
                if (noidungEl) noidungEl.innerText = firstDocumentsNghidinh.subject;
            }
        }
    "#
    .to_string()
}