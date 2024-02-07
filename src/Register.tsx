import {batch, createResource, createSignal, JSX, Show} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import GetSVG from "./GetSVG.tsx";
import PasswordStrengthIndicator from "./PasswordStrengthIndicator.tsx";

/**
 * Register page.
 * @return {JSX.Element} - Div containing the register form.
 */
export default function Register(): JSX.Element {
    const invokeRegister = async (password: string, confirmPassword: string) => await invoke("register", { password: password, confirm_password: confirmPassword }).catch((register_error) => setError(register_error));
    const [input, setInput] = createSignal("");
    const [confirmInput, setConfirmInput] = createSignal("");
    const [password, setPassword] = createSignal<string>();
    const [confirmPassword, setConfirmPassword] = createSignal<string>();

    const [result] = createResource(
        () => [password(), confirmPassword()] as const,
        async ([password, confirmPassword]) => {
            if (password != undefined && confirmPassword != undefined)
                await invokeRegister(password, confirmPassword);
        });
    const [visibility, setVisibility] = createSignal(false);
    const [confirmVisibility, setConfirmVisibility] = createSignal(false);
    const [strength, setStrength] = createSignal(0);
    const [error, setError] = createSignal("");

    return (
        <div class="h-full flex flex-col items-center justify-center">
            <h1 class="font-bold text-3xl">Welcome in Password Manager</h1>
            <p class="text-[16px] mb-3">Please, create your master password.</p>
            <form class="w-80 text-center" onSubmit={(event) => {
                event.preventDefault();
                batch(() => {
                    setPassword(input());
                    setConfirmPassword(confirmInput());
                });
            }}>
                <div class="relative w-full my-3">
                    <input placeholder="Enter your master password" type={visibility() ? 'text' : 'password'} class="pl-4 w-full h-7 rounded-xl pr-10" required
                           onInput={async (event) => {
                               setInput(event.target.value);
                               setStrength(await invoke("password_strength", {password: event.target.value}) as number);
                           }}
                           onInvalid={(event) => {
                               event.preventDefault();
                               setError("Password cannot be empty");
                           }
                    }>
                    </input>
                        <div class="absolute inset-y-0 right-3 flex items-center cursor-pointer" onClick={() => setVisibility(!visibility())} title={(visibility() ? "Hide": "View")+" password"}>
                            <GetSVG name={visibility() ? "eye-slash" : "eye"} class={`h-full p-1 ${visibility() ? 'pr-0.5' : ''}`} />
                        </div>
                </div>
                <div class="relative w-full">
                    <input placeholder="Confirm your master password" type={confirmVisibility() ? 'text' : 'password'} class="pl-4 w-full h-7 rounded-xl pr-10" required
                           onInput={(event) => {
                               setConfirmInput(event.target.value);
                           }}
                           onInvalid={(event) => {
                               event.preventDefault();
                               setError("Password cannot be empty");
                           }
                    }>
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
                <button type="submit" class="mt-2 flex flex-row items-center justify-center gap-2 h-7 w-32 rounded-xl mx-auto bg-[#3578F7] hover:bg-[#2F6AE1] text-[#E9E9E9]" title="Enter">
                    <span class="bg-inherit">Enter</span>
                    <GetSVG name={result.loading ? "spinner" : "right-to-bracket"} class={`w-5 fill-[#E9E9E9] ${result.loading ? 'animate-spin' : ''}`} />
                </button>
            </form>
        </div>
    )
}