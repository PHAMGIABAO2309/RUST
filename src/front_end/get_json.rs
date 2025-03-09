pub fn get_json_code() -> String {
    r#"
        document.addEventListener("DOMContentLoaded", function () {
            fetch("/content")
                .then(response => response.json())
                .then(data => {
                    if (data && data.documents && data.summary) {
                        const documents = data.documents;
                        const summaries = data.summary;

                        if (documents.length > 0) {
                            const firstDoc = documents[0]; // Lấy phần tử đầu tiên

                            let noinhanElements = document.querySelectorAll("[id='noinhan']");
                            noinhanElements.forEach(el => {
                                if (firstDoc.receives) el.innerText = firstDoc.receives;
                            });

                            let hieulucElements = document.querySelectorAll("[id='validitystatus']");
                            hieulucElements.forEach(el => {
                                if (firstDoc.validitystatus) el.innerText = firstDoc.validitystatus;
                            });
                        }

                        if (summaries.length > 0) {
                            const firstSummary = summaries[0];

                            if (firstSummary.oran_name) {
                                let tencoquanEl = document.getElementById("chinhphu");
                                if (tencoquanEl) tencoquanEl.innerText = firstSummary.oran_name;
                            }

                            if (firstSummary.code_number) {
                                let sovanbanEl = document.getElementById("sovanban");
                                if (sovanbanEl) sovanbanEl.innerText = firstSummary.code_number;
                            }

                            if (firstSummary.file_catalog) {
                                let namhinhthanhElements = document.querySelectorAll("[id='namhinhthanh']");
                                namhinhthanhElements.forEach(el => {
                                    el.innerText = firstSummary.file_catalog;
                                });
                            }

                            if (firstSummary.type_name) {
                                let loaiVanBanEl = document.getElementById("loaivanban");
                                if (loaiVanBanEl) loaiVanBanEl.innerText = firstSummary.type_name;
                            }

                            if (firstSummary.full_name) {
                                let nguoikyEl = document.getElementById("nguoiky");
                                if (nguoikyEl) nguoikyEl.innerText = firstSummary.full_name;
                            }

                            if (firstSummary.start_date) {
                                let ngayBatDauEl = document.getElementById("ngaybatdau");
                                if (ngayBatDauEl) ngayBatDauEl.innerText = firstSummary.start_date;
                            }

                            if (firstSummary.end_date) {
                                let ngayKetThucEl = document.getElementById("ngayketthuc");
                                if (ngayKetThucEl) ngayKetThucEl.innerText = firstSummary.end_date;
                            }
                        }

                        let contentHTML = "";
                        documents.forEach(item => {
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
                    } else {
                        let contentEl = document.getElementById("content");
                        if (contentEl) contentEl.innerHTML = "<p>Không có dữ liệu</p>";
                    }
                })
                .catch(error => console.error("Lỗi tải dữ liệu:", error));
        });

    "#.to_string()
}



