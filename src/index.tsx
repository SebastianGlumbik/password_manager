import {createResource, Suspense, Switch, Match, JSX} from "solid-js";
import {render} from "solid-js/web";
import { invoke } from "@tauri-apps/api/tauri";
import Login from './Login.tsx';
import Register from './Register.tsx';
import Main from './Main.tsx';
import Loading from "./Loading.tsx";
import "./style.css"

/*
//TODO: Add in final release
window.addEventListener('contextmenu', function(event) {
    let target = event.target as HTMLInputElement;
    if(target.type !== "password" && target.type !== "text") {
        event.preventDefault();
    }
});
*/

/**
 * Main App component. Based on the window state, it renders the correct component.
 * @return {JSX.Element} - Div containing the main app.
 */
function App(): JSX.Element {
    const [window] = createResource(async () => invoke("initialize_window"));

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
 * Renders the app.
 */
render(App, document.getElementById('root') as HTMLElement);
