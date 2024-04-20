import {createSignal, JSX, Show} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import {appWindow} from '@tauri-apps/api/window'
import GetSVG from "./GetSVG.tsx";
import PasswordStrengthIndicator from "./PasswordStrengthIndicator.tsx";

/**
 * Register page.
 * @return {JSX.Element} - Div containing the register form.
 */
export default function Register(): JSX.Element {
    const [password, setPassword] = createSignal<string>("");
    const [confirmPassword, setConfirmPassword] = createSignal<string>("");
    const [visibility, setVisibility] = createSignal(false);
    const [confirmVisibility, setConfirmVisibility] = createSignal(false);
    const [strength, setStrength] = createSignal(0);
    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal("");

    return (
        <div class="h-full flex flex-col items-center justify-center">
            <h1 class="font-bold text-3xl">Welcome in Password Manager</h1>
            <p class="text-[16px] mb-3">Please, create your master password.</p>
            <form class="w-80 text-center" onSubmit={async (event) => {
                event.preventDefault();
                setLoading(true);
                try {
                    await invoke<void>("register", {password: password(), confirm_password: confirmPassword()})
                    await appWindow.close();
                } catch (e) {
                    setError(e as string);

                } finally {
                    setLoading(false);
                }
            }}>
                <div class="relative w-full my-3">
                    <input placeholder="Enter your master password" type={visibility() ? 'text' : 'password'}
                           class="pl-4 w-full h-7 rounded-xl pr-10"
                           onInput={async (event) => {
                               setPassword(event.target.value);
                               setStrength(await invoke<number>("password_strength", {password: event.target.value}));
                           }}>
                    </input>
                    <div class="absolute inset-y-0 right-3 flex items-center cursor-pointer"
                         onClick={() => setVisibility(!visibility())}
                         title={(visibility() ? "Hide" : "View") + " password"}>
                        <GetSVG name={visibility() ? "eye-slash" : "eye"}
                                class={`h-full p-1 ${visibility() ? 'pr-0.5' : ''}`}/>
                    </div>
                </div>
                <div class="relative w-full">
                    <input placeholder="Confirm your master password" type={confirmVisibility() ? 'text' : 'password'}
                           class="pl-4 w-full h-7 rounded-xl pr-10"
                           onInput={(event) => {
                               setConfirmPassword(event.target.value);
                           }}>
                    </input>
                    <div class="absolute inset-y-0 right-3 flex items-center cursor-pointer"
                         onClick={() => setConfirmVisibility(!confirmVisibility())}
                         title={(confirmVisibility() ? "Hide" : "View") + " password"}>
                        <GetSVG name={confirmVisibility() ? "eye-slash" : "eye"}
                                class={`h-full p-1 ${confirmVisibility() ? 'pr-0.5' : ''}`}/>
                    </div>
                </div>
                <Show when={strength() !== 0}>
                    <div class="mt-1 w-[95%] mx-auto">
                        <PasswordStrengthIndicator strength={strength}/>
                    </div>
                </Show>
                <p class="text-[14px] text-[#EB5545]">{error()}</p>
                <button type="submit"
                        class="mt-2 flex flex-row items-center justify-center gap-2 h-7 w-32 rounded-xl mx-auto bg-[#3578F7] hover:bg-[#2F6AE1] text-[#E9E9E9]"
                        title="Enter" disabled={loading()}>
                    <span class="bg-inherit">Enter</span>
                    <GetSVG name={loading() ? "spinner" : "right-to-bracket"}
                            class={`w-5 fill-[#E9E9E9] ${loading() ? 'animate-spin' : ''}`}/>
                </button>
            </form>
        </div>
    )
}