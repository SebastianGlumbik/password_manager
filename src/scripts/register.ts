import { invoke } from "@tauri-apps/api/tauri";

function show_error(error: string) {
    let errorMessageElement = document.getElementById("error_message");
    if (errorMessageElement) {
        errorMessageElement.textContent = error;
        errorMessageElement.classList.remove("hidden");
    }
}
window.addEventListener("load", async () => {
    document.getElementById("register_form")?.addEventListener("submit", (e) => {
        e.preventDefault();

        let password = (document.getElementById("master_password") as HTMLInputElement).value;
        const confirm_password = (document.getElementById("confirm_master_password") as HTMLInputElement).value;

        invoke("register", { password: password, confirmPassword: confirm_password }).catch((error) => {
            show_error(error);
        })
    });

    let visibility = document.getElementById("visibility");

    visibility?.addEventListener("click", () => {
        let password_input = document.getElementById("master_password");
        if (password_input) {
            const type = password_input.getAttribute("type");

            if (type === "password") {
                password_input.setAttribute("type", "text");
            } else {
                password_input.setAttribute("type", "password");
            }
        }

        let svg_elements = visibility?.getElementsByTagName("svg");
        if(svg_elements) {
            Array.from(svg_elements).forEach(svg => {
                svg.classList.toggle("hidden");
            });
        }
    });

    let visibility_confirm = document.getElementById("visibility_confirm");

    visibility_confirm?.addEventListener("click", () => {
        let password_input = document.getElementById("confirm_master_password");
        if (password_input) {
            const type = password_input.getAttribute("type");

            if (type === "password") {
                password_input.setAttribute("type", "text");
            } else {
                password_input.setAttribute("type", "password");
            }
        }

        let svg_elements = visibility_confirm?.getElementsByTagName("svg");
        if(svg_elements) {
            Array.from(svg_elements).forEach(svg => {
                svg.classList.toggle("hidden");
            });
        }
    });
});