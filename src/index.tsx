import {createResource, Suspense, Switch, Match} from "solid-js";
import {render} from "solid-js/web";
import { invoke } from "@tauri-apps/api/tauri";
import Login from './Login.tsx';
import Register from './Register.tsx';
import Main from './Main.tsx';
import Loading from "./Loading.tsx";
import "./style.css"

function App() {
    const [window] = createResource(async () => invoke("initialize_window"));

    return (
        <Suspense fallback={<Loading/>}>
            <Switch fallback={<div>Error</div>}>
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

render(App, document.getElementById('root') as HTMLElement);
