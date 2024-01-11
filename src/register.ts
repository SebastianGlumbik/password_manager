import { invoke } from "@tauri-apps/api/tauri";
import * as script from "./script.ts";

window.addEventListener("DOMContentLoaded", async () => {
    document.getElementById("register_form")?.addEventListener("submit", (e) => {
        e.preventDefault();

        let password = (document.getElementById("master_password") as HTMLInputElement).value;
        const confirm_password = (document.getElementById("confirm_master_password") as HTMLInputElement).value;

        invoke("register", { password: password, confirmPassword: confirm_password }).catch((error) => {
            script.show_error_message(error);
        })
    });

    script.password_visibility_event_listener("visibility", "master_password");
    script.password_visibility_event_listener("visibility_confirm", "confirm_master_password");
});