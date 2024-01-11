import { invoke } from "@tauri-apps/api/tauri";
import { confirm } from '@tauri-apps/api/dialog';
import * as script from "./script.ts";

window.addEventListener("DOMContentLoaded", async () => {
    document.getElementById("login_form")?.addEventListener("submit", (e) => {
        e.preventDefault();

        const password = (document.getElementById("master_password") as HTMLInputElement).value;

        invoke("login", { password: password }).catch((error) => {
            script.show_error_message(error);
        })
    });

    document.getElementById("start_over")?.addEventListener("click", () => {
        confirm("Are you sure you want to continue? This action will permanently delete all passwords.", { title: "Starting over", type: 'warning' }).then((result) => {
            if (result) {
                invoke("start_over").catch((error) => {
                    script.show_error_message(error);
                })
            }
        });
    });

    script.password_visibility_event_listener("visibility", "master_password");
});