document.addEventListener("DOMContentLoaded", function () {
    const buttons = document.querySelectorAll(".contain_button .item");
    const pages = document.querySelectorAll(".change.page");

    buttons.forEach(button => {
        button.addEventListener("click", () => {
            // Xóa class active_item trên mục đang được chọn trước đó
            document.querySelector('.contain_button .item.active_item')?.classList.remove('active_item');
            
            buttons.forEach(btn => btn.classList.remove("active_item"));
            button.classList.add('active_item');
            // Ẩn tất cả các trang
            pages.forEach(page => page.classList.add('hidden'));

            // Hiển thị trang tương ứng với data-target
            const targetPage = document.getElementById(button.getAttribute("data-target"));
            if (targetPage) {
                targetPage.classList.remove("hidden");
            }
        });
    });
});
