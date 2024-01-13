import { invoke } from "@tauri-apps/api/tauri";
import * as script from "./script.ts";

window.addEventListener("DOMContentLoaded", async () => {
    document.getElementById("login_form")?.addEventListener("submit", (e) => {
        e.preventDefault();

        const password = (document.getElementById("password") as HTMLInputElement).value;

        invoke("login", { password: password }).catch((error) => {
            script.show_error_message(error);
        })
    });

    script.password_visibility_event_listener("visibility", "password");
});