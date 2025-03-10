import { handleDocuments } from './handleDocuments.js';
import { handleSummary } from './handleSummary.js';
import { handleSummaryContent } from './handleSummaryContent.js';

document.addEventListener("DOMContentLoaded", function () {
    fetch("/content")
        .then(response => response.json())
        .then(data => {
            if (data) {
                if (data.documents) handleDocuments(data.documents);
                if (data.summary) handleSummary(data.summary);
                if (data.summary_content) handleSummaryContent(data.summary_content);
            } else {
                let contentEl = document.getElementById("content");
                if (contentEl) contentEl.innerHTML = "<p>Không có dữ liệu</p>";
            }
        })
        .catch(error => console.error("Lỗi tải dữ liệu:", error));
});

console.log("handleDocuments:", handleDocuments);
console.log("handleSummary:", handleSummary);
console.log("handleSummaryContent:", handleSummaryContent);