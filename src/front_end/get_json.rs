pub fn get_json_code() -> String {
    r#"
        document.addEventListener("DOMContentLoaded", function () {
            fetch("/content")
                .then(response => response.json())
                .then(data => {
                    if (data) {
                        if (data.documents) handleDocuments(data.documents);
                        if (data.summary) handleSummary(data.summary);
                        if (data.summary_content) handleSummaryContent(data.summary_content);
                        if (data.list_title) handleListTitle(data.list_title);
                    } else {
                        let contentEl = document.getElementById("content");
                        if (contentEl) contentEl.innerHTML = "<p>Không có dữ liệu</p>";
                    }
                })
                .catch(error => console.error("Lỗi tải dữ liệu:", error));
        });

        function handleDocuments(documents) {
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

            if (documents.length > 0) {
                const firstDoc = documents[0];

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
            }
        }

        function handleSummary(summaries) {
            if (summaries.length === 0) return;

            const firstSummary = summaries[0];

            if (firstSummary.oran_name) {
                let tencoquanEl = document.getElementById("chinhphu");
                if (tencoquanEl) tencoquanEl.innerText = firstSummary.oran_name;
            }

            if (firstSummary.file_nonation) {
                let sohieuvanbannnEl = document.getElementById("sohieuuu");
                if (sohieuvanbannnEl) sohieuvanbannnEl.innerText = firstSummary.file_nonation;
            }

            if (firstSummary.full_name) {
                let nguoikyEl = document.getElementById("nguoikyy");
                if (nguoikyEl) nguoikyEl.innerText = firstSummary.full_name;
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
                let ngayBatDauEl = document.getElementById("ngaybanhanhh");
                if (ngayBatDauEl) {
                    let date = new Date(firstSummary.start_date);
                    let formattedDate = ("0" + date.getDate()).slice(-2) + "/" + 
                                        ("0" + (date.getMonth() + 1)).slice(-2) + "/" + 
                                        date.getFullYear();
                    ngayBatDauEl.innerText = formattedDate;
                }
            }

            if (firstSummary.end_date && firstSummary.end_date !== "N/A") {
                let ngayKetThucEl = document.getElementById("ngayhethieuluc");
                if (ngayKetThucEl) {
                    let date = new Date(firstSummary.end_date);
                    let formattedDate = ("0" + date.getDate()).slice(-2) + "/" + 
                                        ("0" + (date.getMonth() + 1)).slice(-2) + "/" + 
                                        date.getFullYear();
                    ngayKetThucEl.innerText = formattedDate;
                }
            } else {
                let ngayKetThucEl = document.getElementById("ngayhethieuluc");
                if (ngayKetThucEl) ngayKetThucEl.innerText = "Đang cập nhật";
            }
        }

        function handleSummaryContent(summary_content) {
            if (summary_content.length === 0) return;

            const firstSummaryContent = summary_content[0];

            if (firstSummaryContent.content) {
                let contentEl = document.getElementById("tomtat");
                if (contentEl) {
                    let formattedContent = firstSummaryContent.content
                        .replace(/- /g, "<br>- ") 
                        .replace(/TÓM TẮT VĂN BẢN/, "<strong>TÓM TẮT VĂN BẢN</strong>") 
                        .replace(/\n/g, "<br>"); 
                    
                    contentEl.innerHTML = formattedContent;
                }
            }
        }
        function handleListTitle(list_titles) {
            let container = document.getElementById("listContainer");
            container.innerHTML = ""; // Xóa nội dung cũ

            list_titles.forEach((item, index) => {
                let formattedTitle = item.title || "Không có tiêu đề";
                let formattedPath = item.path;
                let formattedStartDate = item.startdate ? new Date(item.startdate).toLocaleDateString("vi-VN") : "Không có";
                let formattedUpdateDate = item.dateupdate ? new Date(item.dateupdate).toLocaleDateString("vi-VN") : "Không có";

                let listItem = `
                    <div class="flex border-t border-gray-300 pt-2 ">
                        <div class="flex space-x-2">
                            <span class="font-bold">${index + 1}</span>
                            <div>
                                 <p class="text-gray-800 font-bold cursor-pointer" 
                       onclick="window.location.href='${formattedPath}'">
                        ${formattedTitle}
                    </p>
                                <div class="flex space-x-4 text-gray-600 mt-2">
                                    <a href="" class="hover:underline">Tóm tắt</a>
                                    <a href="" class="hover:underline">VB liên quan</a>
                                    <a href="" class="hover:underline">Hiệu lực</a>
                                    <a href="" class="hover:underline">Lược đồ</a>
                                    <a href="" class="hover:underline">Tiếng Anh</a>
                                    <a href="" class="hover:underline">Tải về</a>
                                </div>
                            </div>
                        </div>
                        <div class="flex justify-between text-sm text-gray-600 mt-2 ml-40">
                            <div class="flex flex-col space-y-1">
                                <div class="flex justify-between">
                                    <span class="w-20">Ban hành:</span>
                                    <span class="text-orange-600">${formattedStartDate}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="w-20">Áp dụng:</span>
                                    <span class="text-orange-600">Đã biết</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="w-20">Hiệu lực:</span>
                                    <span class="text-orange-600">Đã biết</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="w-20">Cập nhật:</span>
                                    <span class="text-orange-600">${formattedUpdateDate}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                `;
                container.innerHTML += listItem;
            });
        }

    "#.to_string()
}
