document.addEventListener("DOMContentLoaded", () => {
    const form = document.getElementById("registerForm");
    const message = document.getElementById("message");

    form.addEventListener("submit", async (event) => {
        event.preventDefault();

        const formData = new FormData(form);
        const data = new URLSearchParams(formData);

        try {
            const response = await fetch("/register", {
                method: "POST",
                headers: { "Content-Type": "application/x-www-form-urlencoded" },
                body: data,
            });

            const result = await response.json();

            if (result.success) {
                window.location.href = "/hello";
            } else {
                message.style.color = "red";
                message.textContent = result.error;
            }
        } catch (error) {
            console.error("Lỗi:", error);
            message.style.color = "red";
            message.textContent = "Có lỗi xảy ra!";
        }
    });
});
