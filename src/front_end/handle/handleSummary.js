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

    if (firstSummary.start_date) {
        let ngayBatDauEl = document.getElementById("ngaybanhanhh");
        if (ngayBatDauEl) {
            let date = new Date(firstSummary.start_date);
            ngayBatDauEl.innerText = date.toLocaleDateString("vi-VN");
        }
    }

    let ngayKetThucEl = document.getElementById("ngayhethieuluc");
    if (firstSummary.end_date && firstSummary.end_date !== "N/A") {
        let date = new Date(firstSummary.end_date);
        ngayKetThucEl.innerText = date.toLocaleDateString("vi-VN");
    } else {
        ngayKetThucEl.innerText = "Đang cập nhật";
    }
}
