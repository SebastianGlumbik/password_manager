import {createSignal, JSX, Show} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import GetSVG from "./GetSVG.tsx";
import PasswordStrengthIndicator from "./PasswordStrengthIndicator.tsx";
import {message} from "@tauri-apps/api/dialog";

/**
 * Settings page.
 * @return {JSX.Element} - Div containing setting options.
 */
export default function Settings(): JSX.Element {
    const [password, setPassword] = createSignal<string>("");
    const [confirmPassword, setConfirmPassword] = createSignal<string>("");
    const [visibility, setVisibility] = createSignal(false);
    const [confirmVisibility, setConfirmVisibility] = createSignal(false);
    const [strength, setStrength] = createSignal(0);
    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal("");

    return (
        <div class="flex flex-col m-10 items-center">
            <h1 class="font-bold text-3xl mb-3">Settings</h1>
            <div class="flex flex-col py-5 w-full justify-between items-center max-w-6xl rounded-md border border-[#E7E7E7] dark:border-[#3A3A3A] bg-[#F2F2F2] dark:bg-[#2B2B2B]">
                <form class="w-80 text-center" onSubmit={async (event) => {
                    event.preventDefault();
                    setLoading(true);
                    try {
                        await invoke("change_password", { password: password(), confirm_password: confirmPassword() })
                        await message("Password changed successfully!", {title: "Success", type: "info"})
                    } catch (e) {
                        setError(e as string);

                    } finally {
                        setLoading(false);
                    }
                }}>
                    <p class="text-[18px]">Change master password</p>
                    <div class="relative w-full my-3">
                        <input placeholder="Enter new master password" type={visibility() ? 'text' : 'password'} class="pl-4 w-full h-7 rounded-xl pr-10"
                               onInput={async (event) => {
                                   setPassword(event.target.value);
                                   setStrength(await invoke("password_strength", {password: event.target.value}) as number);
                               }}>
                        </input>
                        <div class="absolute inset-y-0 right-3 flex items-center cursor-pointer" onClick={() => setVisibility(!visibility())} title={(visibility() ? "Hide": "View")+" password"}>
                            <GetSVG name={visibility() ? "eye-slash" : "eye"} class={`h-full p-1 ${visibility() ? 'pr-0.5' : ''}`} />
                        </div>
                    </div>
                    <div class="relative w-full">
                        <input placeholder="Confirm new master password" type={confirmVisibility() ? 'text' : 'password'} class="pl-4 w-full h-7 rounded-xl pr-10"
                               onInput={(event) => {
                                   setConfirmPassword(event.target.value);
                               }}>
                        </input>
                        <div class="absolute inset-y-0 right-3 flex items-center cursor-pointer" onClick={() => setConfirmVisibility(!confirmVisibility())} title={(confirmVisibility() ? "Hide": "View")+" password"}>
                            <GetSVG name={confirmVisibility() ? "eye-slash" : "eye"} class={`h-full p-1 ${confirmVisibility() ? 'pr-0.5' : ''}`} />
                        </div>
                    </div>
                    <Show when={strength() !==0}>
                        <div class="mt-1 w-[95%] mx-auto">
                            <PasswordStrengthIndicator strength={strength} />
                        </div>
                    </Show>
                    <p class="text-[14px] text-[#EB5545]">{error()}</p>
                    <button type="submit" class="mt-2 flex flex-row items-center justify-center gap-2 h-7 w-32 rounded-xl mx-auto bg-[#3578F7] hover:bg-[#2F6AE1] text-[#E9E9E9]" title="Enter" disabled={loading()}>
                        <span class="bg-inherit">Change</span>
                        <GetSVG name={loading() ? "spinner" : "pen"} class={`w-4 fill-[#E9E9E9] ${loading() ? 'animate-spin' : ''}`} />
                    </button>
                </form>
            </div>
        </div>
    )
}