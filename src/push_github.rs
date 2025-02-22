use std::process::Command;
/// Hàm tự động đẩy code lên GitHub
pub fn push_to_github() {
    // Thêm tất cả file đã thay đổi
    let status = Command::new("git")
        .args(&["add", "."])
        .status()
        .expect("Không thể chạy lệnh git add");
    if !status.success() {
        eprintln!("Lỗi: git add không thành công");
        return;
    }

    // Commit thay đổi với commit message tự động
    let commit_message = "Auto commit from cargo run";
    let status = Command::new("git")
        .args(&["commit", "-m", commit_message])
        .status()
        .expect("Không thể chạy lệnh git commit");
    if !status.success() {
        // Lưu ý: Có thể không có thay đổi để commit, điều này không cần coi là lỗi nghiêm trọng.
        eprintln!("Lưu ý: git commit không thực hiện được (có thể không có thay đổi mới)");
    }

    // Push lên remote (ở đây giả sử branch chính là 'main')
    let status = Command::new("git")
        .args(&["push", "origin", "master"])
        .status()
        .expect("Không thể chạy lệnh git push");
    if !status.success() {
        eprintln!("Lỗi: git push không thành công");
    } else {
        println!("✅ Đã tự động đẩy code lên GitHub thành công!");
    }
}