import {createResource, JSX} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";

/**
 * Main page. This is where all the magic happens.
 * @return {JSX.Element} - Div containing the main page.
 */
export default function main(): JSX.Element {
    const [data] = createResource(async () => invoke("load_data"));
    return (
        <div class="h-full flex flex-col items-center justify-center">
            <img src="https://www.shuttle.rs/images/blog/crab-builder.png" alt="Crab" class="w-52"></img>
            <p class="font-bold text-2xl">{data() as string}</p>
        </div>
    )
}