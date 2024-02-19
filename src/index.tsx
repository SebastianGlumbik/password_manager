import {createResource, Suspense, Switch, Match, JSX} from "solid-js";
import {render} from "solid-js/web";
import { invoke } from "@tauri-apps/api/tauri";
import Login from './Login.tsx';
import Register from './Register.tsx';
import Main from './Main.tsx';
import Loading from "./Loading.tsx";
import "./style.css"

/**
 * Prevents the context menu from appearing on right-click on the whole app, except for input fields.
 */
window.addEventListener('contextmenu', function(event) {
    if(!(event.target instanceof HTMLInputElement)) {
        event.preventDefault();
    }
});

/**
 * Main App component. Based on backend response, it renders the login, register or main page.
 * @return {JSX.Element} - Div containing the main app.
 */
function App(): JSX.Element {
    const [window] = createResource(async () => invoke<string>("initialize_window"));

    return (
        <Suspense fallback={<Loading/>}>
            <Switch fallback={<div>Not Found</div>}>
            <Match when={window() === "Register"}>
                    <Register />
                </Match>
                <Match when={window() === "Login"}>
                    <Login />
                </Match>
                <Match when={window() === "Main"}>
                    <Main />
                </Match>
            </Switch>
        </Suspense>
    );
}

/**
 * Renders the app to the root div.
 */
render(App, document.getElementById('root') as HTMLElement);