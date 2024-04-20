import {createResource, createSignal, JSX, Show} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import GetSVG from "./GetSVG.tsx";
import PasswordStrengthIndicator from "./PasswordStrengthIndicator.tsx";
import {message} from "@tauri-apps/api/dialog";
import {appWindow} from '@tauri-apps/api/window'

/**
 * Settings page.
 * @return {JSX.Element} - Div containing setting options.
 */
export default function Settings(): JSX.Element {
    return (
        <div class="flex flex-col m-10 items-center gap-3">
            <h1 class="font-bold text-3xl">Settings</h1>
            <SettingsForm>
                <PasswordChangeForm/>
            </SettingsForm>
            <SettingsForm>
                <CloudForm/>
            </SettingsForm>
        </div>
    )
}

function SettingsForm({children}: { children: JSX.Element }) {
    return (
        <div
            class="flex flex-col py-5 w-full justify-between items-center max-w-6xl rounded-md border border-[#E7E7E7] dark:border-[#3A3A3A] bg-[#F2F2F2] dark:bg-[#2B2B2B]">
            {children}
        </div>
    )
}

function PasswordChangeForm() {
    const [password, setPassword] = createSignal<string>("");
    const [confirmPassword, setConfirmPassword] = createSignal<string>("");
    const [visibility, setVisibility] = createSignal(false);
    const [confirmVisibility, setConfirmVisibility] = createSignal(false);
    const [strength, setStrength] = createSignal(0);
    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal("");

    return (
        <form class="w-80 text-center" onSubmit={async (event) => {
            event.preventDefault();
            setLoading(true);
            try {
                await invoke<void>("change_password", {password: password(), confirm_password: confirmPassword()})
                await appWindow.emit("upload");
                setError("");
                setPassword("");
                setConfirmPassword("");
                await message("Password changed successfully!", {title: "Success", type: "info"})
            } catch (e) {
                setError(e as string);

            } finally {
                setLoading(false);
            }
        }}>
            <p class="text-[18px]">Change master password</p>
            <div class="relative w-full my-3">
                <input placeholder="Enter new master password" type={visibility() ? 'text' : 'password'}
                       class="pl-4 w-full h-7 rounded-xl pr-10" value={password()}
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
                <input placeholder="Confirm new master password" type={confirmVisibility() ? 'text' : 'password'}
                       class="pl-4 w-full h-7 rounded-xl pr-10" value={confirmPassword()}
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
                <span class="bg-inherit">Change</span>
                <GetSVG name={loading() ? "spinner" : "pen"}
                        class={`w-4 fill-[#E9E9E9] ${loading() ? 'animate-spin' : ''}`}/>
            </button>
        </form>
    )
}

function CloudForm() {
    const [status, {refetch: refetchStatus}] = createResource(async () => {
        try {
            let result = await invoke<{ address: string, username: string }>("cloud_data");
            setAddress(result.address);
            setUsername(result.username);
            return true
        } catch (e) {
            return false
        }
    })
    const [address, setAddress] = createSignal<string>("");
    const [username, setUsername] = createSignal<string>("");
    const [password, setPassword] = createSignal<string>("");
    const [visibility, setVisibility] = createSignal(false);
    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal("");

    return (
        <form class="w-80 text-center" onSubmit={async (event) => {
            event.preventDefault();
            setLoading(true);
            try {
                if (status()) {
                    await invoke<void>("disable_cloud");
                    await message("Cloud disabled successfully!", {title: "Success", type: "info"})
                } else {
                    await invoke<void>("enable_cloud", {
                        address: address(),
                        username: username(),
                        password: password()
                    });
                }

                setError("");
                setPassword("");
                refetchStatus();
            } catch (e) {
                setError(e as string);

            } finally {
                setLoading(false);
            }
        }}>
            <p class="text-[18px] mb-3">Cloud</p>
            <input placeholder="IP address with port number" type="text" class="pl-4 w-full h-7 rounded-xl pr-10"
                   value={address()}
                   onInput={async (event) => {
                       setAddress(event.target.value);
                   }}>
            </input>
            <input placeholder="Username" type="text" class="pl-4 w-full h-7 rounded-xl pr-10 my-3" value={username()}
                   onInput={async (event) => {
                       setUsername(event.target.value);
                   }}>
            </input>
            <div class="relative w-full">
                <input placeholder="Password" type={visibility() ? 'text' : 'password'}
                       class="pl-4 w-full h-7 rounded-xl pr-10" value={password()}
                       onInput={(event) => {
                           setPassword(event.target.value);
                       }}>
                </input>
                <div class="absolute inset-y-0 right-3 flex items-center cursor-pointer"
                     onClick={() => setVisibility(!visibility())}
                     title={(visibility() ? "Hide" : "View") + " password"}>
                    <GetSVG name={visibility() ? "eye-slash" : "eye"}
                            class={`h-full p-1 ${visibility() ? 'pr-0.5' : ''}`}/>
                </div>
            </div>
            <p class="text-[14px] text-[#EB5545]">{error()}</p>
            <button type="submit"
                    class="mt-2 flex flex-row items-center justify-center gap-2 h-7 w-40 rounded-xl mx-auto bg-[#3578F7] hover:bg-[#2F6AE1] text-[#E9E9E9]"
                    title={!status() ? "Setup" : "Remove"} disabled={loading()}>
                <span class="bg-inherit">{!status() ? "Setup" : "Remove"}</span>
                <GetSVG name={loading() ? "spinner" : (!status() ? "cloud" : "xmark")}
                        class={`w-4 fill-[#E9E9E9] ${loading() ? 'animate-spin' : ''}`}/>
            </button>
        </form>
    )
}