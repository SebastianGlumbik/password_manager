import { showMenu } from "tauri-plugin-context-menu";
import {createResource, createSignal, For, JSX, Match, onCleanup, onMount, Show, Suspense, Switch} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import Loading from "./Loading.tsx";
import GetSVG from "./GetSVG.tsx";
import {Record, KindSVG, editSignal} from "./Model.tsx";
import RecordDetail from "./RecordDetail.tsx";
import {confirm, message} from '@tauri-apps/api/dialog';
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import Settings from "./Settings.tsx";

/**
 * Main page. This is where all the magic happens.
 * @return {JSX.Element} - Div containing the main page.
 */
export default function Main(): JSX.Element {
    const [allRecords, {refetch: refetchAllRecords }] = createResource(async (): Promise<Record[]> => (await invoke("get_all_records") as Record[]).map((item: Record) => new Record(item.title, item.subtitle, item.category, item.created, item.last_modified, item.id)));
    const [compromisedOnly, setCompromisedOnly] = createSignal(false);
    const [compromisedRecords] = createResource(allRecords,async (): Promise<number[]> => (await invoke("get_compromised_records") as number[]));
    const compromisedExists = () => !compromisedRecords.loading && compromisedRecords()?.length as number > 0;
    const [search, setSearch] = createSignal("");
    const filteredRecords = () => allRecords()?.filter(record => compromisedOnly() ? compromisedRecords()?.includes(record.id as number) : true ).filter(record => (record.title.toLowerCase().includes(search().toLowerCase()) || record.subtitle.toLowerCase().includes(search().toLowerCase()) || record.category.toLowerCase().includes(search().toLowerCase())));
    const [selected, setSelected] = createSignal<Record | "Settings" |  undefined>((filteredRecords()?.[0]) as Record);
    const [cloud, {refetch: uploadToCloud }] = createResource(async (): Promise<string> => invoke("save_to_cloud"));

    const [edit, setEdit] = editSignal;
    const select = async (selection: Record | "Settings" |  undefined) => {
        if(edit()) {
            const confirmed = await confirm("You have unsaved changes. Are you sure you want to continue?",{ title: 'Unsaved changes', type: 'warning' });
            if(!confirmed) {
                return false;
            }
        }
        if (selection !== selected()){
            setEdit(false);
            setSelected(selection);
        }
        return true;
    }

    let unlisten : UnlistenFn | undefined = undefined;

    onMount(async () => {
        unlisten = await listen("new_record", async (event) => {
            let temp : Record = event.payload as Record;
            await select(new Record(temp.title, temp.subtitle, temp.category, temp.created, temp.last_modified, temp.id));
            setEdit(true);
        });
    });

    onCleanup(async () => {
        if (unlisten)
            unlisten();
    });

    const refetchAll = () => {
        refetchAllRecords();
        uploadToCloud();
    }

    return (
        <div class="h-full flex flex-row">
            <div class="relative flex flex-col w-2/5 min-w-56 max-w-96 p-5 items-center bg-[#F2F2F2] dark:bg-[#383838] overflow-auto" onClick={async (event) => {
                if (event.target.isSameNode(event.currentTarget)) await select( undefined );
            }
            }>
                <Suspense fallback={<Loading/>}>
                    <Show when={allRecords()?.length as number != 0} fallback={
                        <div class="absolute h-full bottom-0 flex flex-col items-center justify-center">
                            <p class="font-bold">No records</p>
                            <p class="text-center text-[#828282] dark:text-[#9F9F9F]">You can add them in the window menu.</p>
                        </div>
                    }>
                        <div class="relative w-full mb-3 z-10">
                            <input class="w-full h-8 rounded-md border border-[#E4E4E4] dark:border-[#4E4E4E] bg-[#E9E9E9] dark:bg-[#454545] text-[14px] pl-9" type="text" placeholder="Search" onInput={(event) => setSearch(event.currentTarget.value)}></input>
                            <div class="absolute inset-y-0 left-1 flex items-center">
                                <GetSVG name={"magnifying-glass"} class={"h-full p-2"} />
                            </div>
                        </div>
                        <div class="grid grid-cols-1 w-full rounded-md border border-[#E4E4E4] dark:border-[#4E4E4E] bg-[#E9E9E9] dark:bg-[#454545] mb-3 z-10">
                            <div class={`cursor-pointer rounded-t-md ${compromisedOnly() ? 'bg-[#0064E1] text-[#E9E9E9]' : ''}`} onClick={() => setCompromisedOnly(!compromisedOnly())
                            }>
                                <div class="flex flex-row px-2 py-0.5 items-center my-1">
                                    <div class={`mr-3 shadow-md rounded-md bg-[#98989D] min-w-8 w-8 min-h-8 h-8 flex items-center justify-center ${compromisedExists() ? 'bg-red-600' : 'bg-green-600'}`}>
                                        <GetSVG name={compromisedExists() ? "circle-exclamation" : "circle-check"} class={"w-4 h-4 fill-white"} />
                                    </div>
                                    <div class="truncate">
                                        <div class="text-[14px] truncate">
                                            Security check
                                        </div>
                                        <div class="text-[12px] text-[#828282] dark:text-[#9F9F9F] truncate">
                                            {compromisedExists() ? compromisedRecords()?.length + " problem" + ((compromisedRecords()?.length as number > 1) ? "s" : "") +  " has been found" : 'Everything is fine'}
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="w-[90%] mx-auto h-[1px] bg-[#E4E4E4] dark:bg-[#4E4E4E]"></div>
                            <div class={`cursor-pointer rounded-b-md ${selected() === "Settings" ? 'bg-[#0064E1] text-[#E9E9E9]' : ''}`} onClick={async () => await select((selected() === "Settings") ? undefined : "Settings")}>
                                <div class="flex flex-row px-2 py-0.5 items-center my-1">
                                    <div class="mr-3 shadow-md rounded-md bg-[#98989D] min-w-8 w-8 min-h-8 h-8 flex items-center justify-center">
                                        <GetSVG name={cloud.loading ? "cloud-arrow-up" : "cloud"} class={`w-4 h-4 fill-white ${cloud.loading ? 'animate-bounce' : ''}`} />
                                    </div>
                                    <div class="truncate">
                                        <div class="text-[14px] truncate">
                                            Cloud settings
                                        </div>
                                        <div class="text-[12px] text-[#828282] dark:text-[#9F9F9F] truncate">
                                            <Suspense fallback={"Syncing..."}>
                                                <Show when={cloud.state == "errored"} fallback={cloud()}>
                                                    {"Failed to sync"}
                                                </Show>
                                            </Suspense>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <Show when={filteredRecords()?.length as number != 0} fallback={
                            <div class="absolute h-full bottom-0 flex items-center justify-center text-[#828282] dark:text-[#9F9F9F]">
                                <p>No match</p>
                            </div>
                        }>
                            <div class="grid grid-cols-1 w-full rounded-md border border-[#E4E4E4] dark:border-[#4E4E4E] bg-[#E9E9E9] dark:bg-[#454545]">
                                <For each={filteredRecords()}>{(item, index) =>
                                    <>
                                    <div class={`cursor-pointer ${(selected() as Record)?.id === item.id ? 'bg-[#0064E1] text-[#E9E9E9]' : ''} last:rounded-b-md first:rounded-t-md`} onClick={async () => await select((selected() as Record)?.id === item.id ? undefined : item)}
                                         onContextMenu={async (event) => {
                                        event.preventDefault();
                                        if (await select(item))
                                            await showMenu({
                                                items: [{
                                                    label: `Delete ${item.title}`,
                                                    disabled: false,
                                                    event: async (event) => {
                                                        const confirmed = await confirm("Are you sure you want to delete " + event?.payload.record.title + "?",{title: "Delete " + event?.payload.record.title, type: "warning"});
                                                        if(!confirmed) {
                                                            return;
                                                        }
                                                        try {
                                                            await invoke("delete_record", {record: event?.payload.record});
                                                        }
                                                        catch (e) {
                                                            await message(e as string, { title: 'Error', type: 'error' });
                                                        }
                                                        refetchAll();
                                                        await select(undefined);
                                                    },
                                                    payload: {record: item},
                                                }]
                                            });
                                    }}>
                                        <div class="flex flex-row px-2 py-0.5 items-center my-1">
                                            <div class="mr-3 shadow-md rounded-md bg-[#98989D] min-w-8 w-8 min-h-8 h-8 flex items-center justify-center">
                                                <GetSVG name={KindSVG(item.category)} class="w-4 h-4 fill-white"/>
                                            </div>
                                            <div class="truncate">
                                                <div class="text-[14px] truncate">
                                                    {item.title}
                                                </div>
                                                <div class={`text-[12px] text-[#828282] dark:text-[#9F9F9F] ${(selected() as Record)?.id === item.id ? 'text-[#D3D3D3] dark:text-[#D3D3D3]' : ''} truncate`}>
                                                    {item.subtitle}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                        <Show when={index() < (filteredRecords()?.length as number - 1)}>
                                            <div class="w-[90%] mx-auto h-px bg-[#E4E4E4] dark:bg-[#4E4E4E]"></div>
                                        </Show>
                                    </>
                                }</For>
                            </div>
                        </Show>
                    </Show>
                </Suspense>
            </div>
            <div class="h-full w-px bg-[#E4E4E4] dark:bg-black"></div>
            <div class={"relative w-full h-full overflow-y-scroll"}>
                <Suspense fallback={<Loading/>}>
                    <Switch fallback={
                        <div class="flex flex-col items-center justify-center h-full">
                            <p class="text-xl text-[#828282] dark:text-[#9F9F9F]">Select a record to view it</p>
                        </div>
                    }>
                        <Match when={selected() === "Settings"}>
                            <Settings />
                        </Match>
                        <Match when={selected() instanceof Record}>
                            <RecordDetail record={selected as () => Record} refresh={refetchAll} />
                        </Match>
                    </Switch>
                </Suspense>
            </div>
        </div>
    )
}