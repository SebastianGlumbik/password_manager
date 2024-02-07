import {createResource, createSignal, JSX} from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import GetSVG from "./GetSVG.tsx";
// @ts-ignore
import logo from "./assets/logo.png";


/**
 * Login page
 * @return {JSX.Element} - Div containing the login form.
 */
export default function Login(): JSX.Element {
    const invokeLogin = async (password: string) => await invoke("login", { password: password }).catch((login_error) => setError(login_error));
    const [input, setInput] = createSignal("");
    const [password, setPassword] = createSignal<string>();
    const [result] = createResource(password, invokeLogin);
    const [visibility, setVisibility] = createSignal(false);
    const [error, setError] = createSignal("");

    return (
        <div class="h-full flex flex-col items-center justify-center gap-0">
            <img src={logo} alt="Password Manager" class="w-60 mb-4" draggable="false"></img>
                <form class="flex flex-row gap-1 w-72" onSubmit={(event) => {
                    event.preventDefault();
                    setPassword(input());
                }}>
                    <div class="relative w-full">
                        <input placeholder="Enter your master password" type={visibility() ? 'text' : 'password'} class="pl-3 w-full h-7 rounded-xl pr-10" onChange={(event) => setInput(event.currentTarget.value)}></input>
                            <div class="absolute inset-y-0 right-2 flex items-center cursor-pointer" onClick={() => setVisibility(!visibility())} title={(visibility() ? "Hide": "View")+" password"}>
                                <GetSVG name={visibility() ? "eye-slash" : "eye"} class={`h-full p-1 ${visibility() ? 'pr-0.5' : ''}`} />
                            </div>
                    </div>
                    <button type="submit" class="w-7 h-7 p-0.5" title="Login">
                        <GetSVG name={result.loading ? "spinner" : "right-to-bracket"} class={result.loading ? "animate-spin" : ""} />
                    </button>
                </form>
            <div class="h-[21px] mt-0.5 text-center">
                <p class="text-[14px] text-[#EB5545]">{error()}</p>
            </div>
        </div>
    )
}