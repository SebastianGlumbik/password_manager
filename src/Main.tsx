import { showMenu } from "tauri-plugin-context-menu";
import {createResource, createSignal, For, JSX, Match, onCleanup, Show, Suspense, Switch} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import Loading from "./Loading.tsx";

class Record {
    id?: number;
    title: string;
    subtitle: string;
    category: string;
    created?: string;
    last_modified?: string;

    constructor(title: string, subtitle: string, category: string, created?: string, last_modified?: string, id?: number) {
        this.id = id;
        this.title = title;
        this.subtitle = subtitle;
        this.category = category;
        this.created = created;
        this.last_modified = last_modified;
    }
}


class Content {
    id?: number;
    label: string;
    position: number;
    required: boolean;
    kind: string;
    value?: number | string;

    constructor(label: string, position: number, required: boolean, kind: string, value?: number | string, id?: number) {
        this.id = id;
        this.label = label;
        this.position = position;
        this.required = required;
        this.kind = kind;
        this.value = value;
    }
}

class Cloud {

}

function GetRecordTitleSvg(props: {category: string, style: string}) {
    return (
            <Switch fallback={
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class={props.style}>
                    <path
                        d="M176 56V96H336V56c0-4.4-3.6-8-8-8H184c-4.4 0-8 3.6-8 8zM128 96V56c0-30.9 25.1-56 56-56H328c30.9 0 56 25.1 56 56V96v32V480H128V128 96zM64 96H96V480H64c-35.3 0-64-28.7-64-64V160c0-35.3 28.7-64 64-64zM448 480H416V96h32c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64z"/>
                </svg>
            }>
                <Match when={props.category === "Login"}>
                    <svg xmlns="http://www.w3.org/2000/svg"
                         viewBox="0 0 512 512" class={props.style}>
                        <path
                            d="M352 256c0 22.2-1.2 43.6-3.3 64H163.3c-2.2-20.4-3.3-41.8-3.3-64s1.2-43.6 3.3-64H348.7c2.2 20.4 3.3 41.8 3.3 64zm28.8-64H503.9c5.3 20.5 8.1 41.9 8.1 64s-2.8 43.5-8.1 64H380.8c2.1-20.6 3.2-42 3.2-64s-1.1-43.4-3.2-64zm112.6-32H376.7c-10-63.9-29.8-117.4-55.3-151.6c78.3 20.7 142 77.5 171.9 151.6zm-149.1 0H167.7c6.1-36.4 15.5-68.6 27-94.7c10.5-23.6 22.2-40.7 33.5-51.5C239.4 3.2 248.7 0 256 0s16.6 3.2 27.8 13.8c11.3 10.8 23 27.9 33.5 51.5c11.6 26 20.9 58.2 27 94.7zm-209 0H18.6C48.6 85.9 112.2 29.1 190.6 8.4C165.1 42.6 145.3 96.1 135.3 160zM8.1 192H131.2c-2.1 20.6-3.2 42-3.2 64s1.1 43.4 3.2 64H8.1C2.8 299.5 0 278.1 0 256s2.8-43.5 8.1-64zM194.7 446.6c-11.6-26-20.9-58.2-27-94.6H344.3c-6.1 36.4-15.5 68.6-27 94.6c-10.5 23.6-22.2 40.7-33.5 51.5C272.6 508.8 263.3 512 256 512s-16.6-3.2-27.8-13.8c-11.3-10.8-23-27.9-33.5-51.5zM135.3 352c10 63.9 29.8 117.4 55.3 151.6C112.2 482.9 48.6 426.1 18.6 352H135.3zm358.1 0c-30 74.1-93.6 130.9-171.9 151.6c25.5-34.2 45.2-87.7 55.3-151.6H493.4z"/>
                    </svg>
                </Match>
                <Match when={props.category === "BankCard"}>
                    <svg xmlns="http://www.w3.org/2000/svg"
                         viewBox="0 0 576 512" class={props.style}>
                        <path
                            d="M64 32C28.7 32 0 60.7 0 96v32H576V96c0-35.3-28.7-64-64-64H64zM576 224H0V416c0 35.3 28.7 64 64 64H512c35.3 0 64-28.7 64-64V224zM112 352h64c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16zm112 16c0-8.8 7.2-16 16-16H368c8.8 0 16 7.2 16 16s-7.2 16-16 16H240c-8.8 0-16-7.2-16-16z"/>
                    </svg>
                </Match>
                <Match when={props.category === "Note"}>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class={props.style}>
                        <path
                            d="M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H288V368c0-26.5 21.5-48 48-48H448V96c0-35.3-28.7-64-64-64H64zM448 352H402.7 336c-8.8 0-16 7.2-16 16v66.7V480l32-32 64-64 32-32z"/>
                    </svg>
                </Match>
            </Switch>
    )
}

/**
 * Main page. This is where all the magic happens.
 * @return {JSX.Element} - Div containing the main page.
 */
export default function Main(): JSX.Element {
    const [allRecords, {refetch: refetchAllRecords }] = createResource(async (): Promise<Record[]> => (await invoke("get_all_records") as Record[]).map((item: Record) => new Record(item.title, item.subtitle, item.category, item.created, item.last_modified, item.id)));
    const [compromisedOnly, setCompromisedOnly] = createSignal(false);
    const [compromisedRecords, {refetch: refetchCompromisedRecords }] = createResource(async (): Promise<Record[]> => invoke("get_compromised_records"));
    const compromisedExists = () => !compromisedRecords.loading && compromisedRecords()?.length as number > 0;
    const [search, setSearch] = createSignal("");
    const filteredRecords = () => (compromisedOnly() ? compromisedRecords() : allRecords())?.filter(record => (record.title.includes(search()) || record.subtitle.includes(search())));
    const [selected, setSelected] = createSignal<Record | Cloud |  undefined>(undefined);
    const [cloud, {refetch: uploadToCloud }] = createResource(async (): Promise<string> => invoke("save_to_cloud"));

    const refetchAll = () => {
        refetchAllRecords();
        refetchCompromisedRecords();
    }

    return (
        <div class="h-full flex flex-row">
            <div class="relative flex flex-col w-1/3 min-w-56 max-w-96 p-5 items-center bg-[#F2F2F2] dark:bg-[#383838] overflow-scroll" onClick={(event) => {
                if (event?.target.isSameNode(event?.currentTarget)) setSelected( undefined );

            }
            }>
                <Suspense fallback={<Loading/>}>
                        <div class="relative w-full mb-3 z-10">
                            <input class="w-full h-8 rounded-md border border-[#E4E4E4] dark:border-[#4E4E4E] bg-[#E9E9E9] dark:bg-[#454545] text-[14px] pl-9" type="text" placeholder="Search" onInput={(event) => setSearch(event.currentTarget.value)}></input>
                            <div class="absolute inset-y-0 left-1 flex items-center">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="h-full p-2"><path d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/></svg>
                            </div>
                        </div>
                        <div class="grid grid-cols-1 w-full rounded-md border border-[#E4E4E4] dark:border-[#4E4E4E] bg-[#E9E9E9] dark:bg-[#454545] mb-3 z-10">
                            <div class={`cursor-pointer rounded-t-md ${compromisedOnly() ? 'bg-[#0064E1] text-[#E9E9E9]' : ''}`} onClick={(_) => {
                                setCompromisedOnly(!compromisedOnly());
                            }
                            }>
                                <div class="flex flex-row px-2 py-0.5 items-center my-1">
                                    <div class={`mr-3 shadow-md rounded-md bg-[#98989D] min-w-8 w-8 min-h-8 h-8 flex items-center justify-center ${compromisedExists() ? 'bg-red-600' : 'bg-green-600'}`}>
                                        <Show when={compromisedExists()} fallback={
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-4 h-4 fill-white"><path d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209L241 337c-9.4 9.4-24.6 9.4-33.9 0l-64-64c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l47 47L335 175c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9z"/></svg>
                                        }>
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-4 h-4 fill-white"><path d="M256 32c14.2 0 27.3 7.5 34.5 19.8l216 368c7.3 12.4 7.3 27.7 .2 40.1S486.3 480 472 480H40c-14.3 0-27.6-7.7-34.7-20.1s-7-27.8 .2-40.1l216-368C228.7 39.5 241.8 32 256 32zm0 128c-13.3 0-24 10.7-24 24V296c0 13.3 10.7 24 24 24s24-10.7 24-24V184c0-13.3-10.7-24-24-24zm32 224a32 32 0 1 0 -64 0 32 32 0 1 0 64 0z"/></svg>
                                        </Show>
                                    </div>
                                    <div class="truncate">
                                        <div class="text-[14px] truncate">
                                            Security check
                                        </div>
                                        <div class="text-[12px] font-thin truncate">
                                            {compromisedExists() ? "A problem has been found" : 'Everything is fine'}
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="w-[90%] mx-auto h-[1px] bg-[#E4E4E4] dark:bg-[#4E4E4E]"></div>
                            <div class={`cursor-pointer rounded-b-md ${selected() instanceof Cloud ? 'bg-[#0064E1] text-[#E9E9E9]' : ''}`} onClick={() => setSelected((selected() instanceof Cloud) ? undefined : new Cloud())}>
                                <div class="flex flex-row px-2 py-0.5 items-center my-1">
                                    <div class="mr-3 shadow-md rounded-md bg-[#98989D] min-w-8 w-8 min-h-8 h-8 flex items-center justify-center">
                                        <Show when={cloud.loading} fallback={
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512" class="w-4 h-4 fill-white"><path d="M0 336c0 79.5 64.5 144 144 144H512c70.7 0 128-57.3 128-128c0-61.9-44-113.6-102.4-125.4c4.1-10.7 6.4-22.4 6.4-34.6c0-53-43-96-96-96c-19.7 0-38.1 6-53.3 16.2C367 64.2 315.3 32 256 32C167.6 32 96 103.6 96 192c0 2.7 .1 5.4 .2 8.1C40.2 219.8 0 273.2 0 336z"/></svg>
                                        }>
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512" class="w-4 h-4 fill-white animate-bounce"><path d="M144 480C64.5 480 0 415.5 0 336c0-62.8 40.2-116.2 96.2-135.9c-.1-2.7-.2-5.4-.2-8.1c0-88.4 71.6-160 160-160c59.3 0 111 32.2 138.7 80.2C409.9 102 428.3 96 448 96c53 0 96 43 96 96c0 12.2-2.3 23.8-6.4 34.6C596 238.4 640 290.1 640 352c0 70.7-57.3 128-128 128H144zm79-217c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l39-39V392c0 13.3 10.7 24 24 24s24-10.7 24-24V257.9l39 39c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-80-80c-9.4-9.4-24.6-9.4-33.9 0l-80 80z"/></svg>
                                        </Show>
                                    </div>
                                    <div class="truncate">
                                        <div class="text-[14px] truncate">
                                            Cloud settings
                                        </div>
                                        <div class="text-[12px] font-thin truncate">
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
                    <Show when={allRecords()?.length as number != 0} fallback={
                        <div class="absolute h-full bottom-0 flex flex-col items-center justify-center">
                            <p class="font-bold">No records</p>
                            <p class="text-center font-thin">You can add them in the window menu.</p>
                        </div>
                    }>
                        <Show when={filteredRecords()?.length as number != 0} fallback={
                            <div class="absolute h-full bottom-0 flex items-center justify-center">
                                <p>No match</p>
                            </div>
                        }>
                            <div class="grid grid-cols-1 w-full rounded-md border border-[#E4E4E4] dark:border-[#4E4E4E] bg-[#E9E9E9] dark:bg-[#454545]">
                                <For each={filteredRecords()}>{(item, index) =>
                                    <>
                                    <div class={`cursor-pointer ${selected() === item ? 'bg-[#0064E1] text-[#E9E9E9]' : ''} ${(index() === (filteredRecords()?.length as number - 1)) ? 'rounded-b-md' : ''} ${(index() === 0) ? 'rounded-t-md' : ''}`} onClick={() => setSelected((selected() === item) ? undefined : item)}
                                         onContextMenu={async (event) => {
                                        event.preventDefault();
                                        setSelected(item);
                                        await showMenu({
                                            items: [{
                                                label: "Delete",
                                                disabled: false,
                                                event: async (event) => {
                                                    console.log(event);
                                                    await invoke("delete_record", {record: event?.payload.record}).then(() => {
                                                        refetchAll();
                                                        setSelected(undefined);
                                                    });
                                                },
                                                payload: {record: item},
                                            }]
                                        });
                                    }}>
                                        <div class="flex flex-row px-2 py-0.5 items-center my-1">
                                            <div class="mr-3 shadow-md rounded-md bg-[#98989D] min-w-8 w-8 min-h-8 h-8 flex items-center justify-center">
                                                <GetRecordTitleSvg category={item.category as string} style="w-4 h-4 fill-white"/>
                                            </div>
                                            <div class="truncate">
                                                <div class="text-[14px] truncate">
                                                    {item.title}
                                                </div>
                                                <div class="text-[12px] font-thin truncate">
                                                    {item.subtitle}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                        <Show when={index() < (filteredRecords()?.length as number - 1)}>
                                            <div class="w-[90%] mx-auto h-[1px] bg-[#E4E4E4] dark:bg-[#4E4E4E]"></div>
                                        </Show>
                                    </>
                                }</For>
                            </div>
                        </Show>
                    </Show>
                </Suspense>
            </div>
            <div class="h-full w-px bg-[#E4E4E4] dark:bg-black"></div>
            <div class="flex-grow flex items-center justify-center">
                <Suspense fallback={<Loading/>}>
                    <Switch fallback={
                        <p class="text-xl font-thin">Select a record to view it</p>
                    }>
                        <Match when={selected() instanceof Cloud}>
                            <CloudSettings />
                        </Match>
                        <Match when={selected() instanceof Record}>
                            <RecordDetail record={selected as () => Record} uploadToCloud={uploadToCloud}/>
                        </Match>
                    </Switch>
                </Suspense>
            </div>
        </div>
    )
}

function RecordDetail(props: {record: () => Record, uploadToCloud: () => void}) {
    const [_data] = createResource(props.record,async (): Promise<Content[]> => invoke("get_all_content_for_record", { record: props.record()}));

    onCleanup(() => {
        props.uploadToCloud();
    });

    return (
        <p class="text-xl" onClick={async _ => {
            let content : Content[] = [
                {
                    label: "Text",
                    position: 1,
                    required: true,
                    kind: "TOTPSecret"
                },
                {
                    label: "Text",
                    position: 2,
                    required: true,
                    kind: "Text",
                    value: "Hello world"
                }];
            console.log(content);
            console.log(_data());
            await invoke("save_test", {record: props.record(), content: content}).catch((error) => console.log(error));
        }}>{props.record().id}</p>
    )
}

function CloudSettings() {
    return (
        <p class="text-xl">Cloud settings</p>
    )
}