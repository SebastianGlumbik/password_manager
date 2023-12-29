import { invoke } from "@tauri-apps/api/tauri";
import { confirm } from '@tauri-apps/api/dialog';

function show_error(error: string) {
    let errorMessageElement = document.getElementById("error_message");
    if (errorMessageElement) {
        errorMessageElement.textContent = error;
        errorMessageElement.classList.remove("hidden");
    }
}
window.addEventListener("load", async () => {
    document.getElementById("login_form")?.addEventListener("submit", (e) => {
        e.preventDefault();

        const password = (document.getElementById("master_password") as HTMLInputElement).value;

        invoke("login", { password: password }).catch((error) => {
            show_error(error);
        })
    });

    document.getElementById("start_over")?.addEventListener("click", () => {
        confirm("Are you sure you want to continue? This action will permanently delete all passwords.", { title: "Starting over", type: 'warning' }).then((result) => {
            if (result) {
                invoke("start_over").catch((error) => {
                    show_error(error);
                })
            }
        });
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
});