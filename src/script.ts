//document.addEventListener('contextmenu', event => event.preventDefault());

export function show_error_message(error: string, error_element_id: string = "error_message") {
    let errorMessageElement = document.getElementById(error_element_id);
    if (errorMessageElement) {
        errorMessageElement.textContent = error;
        errorMessageElement.classList.remove("hidden");
    }
}

export function password_visibility_event_listener(visibility_id: string, password_id: string) {
    let visibility = document.getElementById(visibility_id);

    visibility?.addEventListener("click", () => {
        let password_input = document.getElementById(password_id);
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
}