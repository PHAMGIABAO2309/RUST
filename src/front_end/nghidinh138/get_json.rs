pub fn get_json_code() -> String {
    r#"
        document.addEventListener("DOMContentLoaded", function () {
            fetch("/content")
                .then(response => response.json())
                .then(data => {
                    if (data) {
                        if (data.documents_nghidinh138) handleDocuments_Nghidinh138(data.documents_nghidinh138);
                        
                    } else {
                        let contentEl = document.getElementById("content");
                        if (contentEl) contentEl.innerHTML = "<p>Không có dữ liệu</p>";
                    }
                })
                .catch(error => console.error("Lỗi tải dữ liệu:", error));
        });

        function handleDocuments_Nghidinh138(documents_nghidinh138) {
            let contentHTML = "";
            documents_nghidinh138.forEach(item => {
                if (item.subject) {
                    let formattedContent = item.subject.replace(/\r\n/g, '<br><br>');
                    formattedContent = formattedContent.replace(/(NGHỊ .*?)<br>/g, '<br><br><h2 style="text-align: center;"><strong>$1</strong></h2><br>');
                    formattedContent = formattedContent.replace(/(Mục .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2>');
                    formattedContent = formattedContent.replace(/(Chương .*?)<br>/g, '<h2 style="text-align: center;"><strong>$1</strong></h2><br>');
                    formattedContent = formattedContent.replace(/(Điều \d+\..*?)<br><br>/g, '<strong>$1</strong><br><br>');

                    contentHTML += `<p>${formattedContent}</p>`;
                }
            });

            let contentEl = document.getElementById("content");
            if (contentEl) {
                contentEl.innerHTML = contentHTML;
            }

            if (documents_nghidinh138.length > 0) {
                const firstDoc = documents_nghidinh138[0];

                let noinhanElements = document.querySelectorAll("[id='noinhan']");
                noinhanElements.forEach(el => {
                    if (firstDoc.receives) el.innerText = firstDoc.receives;
                });

                let hieulucElements = document.querySelectorAll("[id='validitystatus']");
                hieulucElements.forEach(el => {
                    if (firstDoc.validitystatus) el.innerText = firstDoc.validitystatus;
                });

                if (firstDoc.codenumber) {
                    let sovanbanEl = document.getElementById("sovanban");
                    if (sovanbanEl) sovanbanEl.innerText = firstDoc.codenumber;
                }

                if (firstDoc.filecatalog) {
                    let namhinhthanhElements = document.querySelectorAll("[id='namhinhthanh']");
                    namhinhthanhElements.forEach(el => {
                        el.innerText = firstDoc.filecatalog;
                    });
                }
                if (firstDoc.title) {
                    let titleEl = document.getElementById("title_nghidinh138");
                    if(titleEl) titleEl.innerText = firstDoc.title;
                }
            }
        }

    "#.to_string()
}