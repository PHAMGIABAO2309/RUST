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
