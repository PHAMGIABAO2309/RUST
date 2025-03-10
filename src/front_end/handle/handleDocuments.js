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

        document.querySelectorAll("[id='noinhan']").forEach(el => {
            if (firstDoc.receives) el.innerText = firstDoc.receives;
        });

        document.querySelectorAll("[id='validitystatus']").forEach(el => {
            if (firstDoc.validitystatus) el.innerText = firstDoc.validitystatus;
        });

        let sovanbanEl = document.getElementById("sovanban");
        if (sovanbanEl && firstDoc.codenumber) sovanbanEl.innerText = firstDoc.codenumber;

        document.querySelectorAll("[id='namhinhthanh']").forEach(el => {
            if (firstDoc.filecatalog) el.innerText = firstDoc.filecatalog;
        });
    }
}
