import {Content, KindSVG, Record, editSignal} from "./Model.tsx";
import {createResource, createSignal, For, JSX, Match, onCleanup, Show, Switch} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import GetSVG from "./GetSVG.tsx";
import { writeText } from '@tauri-apps/api/clipboard';
import {showMenu} from "tauri-plugin-context-menu";
import { Item } from "tauri-plugin-context-menu/dist/types";
import PasswordStrengthIndicator from "./PasswordStrengthIndicator.tsx";
import {confirm, message} from "@tauri-apps/api/dialog";

/**
 * Shows a context menu with a single item to copy the given text to the clipboard.
 * @param text - The text to be copied to the clipboard.
 */
async function copyTextToClipboard(text: string) {
    await showMenu({
        items: [{
            label: "Copy " + text,
            disabled: false,
            event: async (event) => await writeText(event?.payload.text),
            payload: {text: text},
        }]
    });
}

/**
 * Record detail component
 * @param {Record} record - The record to be displayed.
 * @param {Function} refresh - Function to refresh the records list.
 * @return {JSX.Element} - Div containing the record detail.
 */

export default function RecordDetail({record,refresh}: {record: () => Record, refresh: () => void}): JSX.Element {
    //let record = props.record;
    const [allContent,{mutate: newContent, refetch: refetchContent}] = createResource(record,async (): Promise<Content[]> => (await invoke("get_all_content_for_record", { record: record()}) as Content[]).map((item: Content) => new Content(item.label, item.position, item.required, item.kind, item.value, item.id)).sort((a, b) => a.position - b.position));
    const [edit, setEdit] = editSignal;
    const [error, setError] = createSignal("");

    return (
        <div id="record-detail" class="m-10 flex justify-center">
            <form class="grid grid-cols-1 min-h-full w-full max-w-6xl rounded-md border border-[#E7E7E7] dark:border-[#3A3A3A] bg-[#F2F2F2] dark:bg-[#2B2B2B]" onSubmit={async (event) => {
                event.preventDefault();
                if(edit()) {
                    try {
                        record().id = (await invoke("save_record",{record: record(), content: allContent()}) as Record).id;
                    }
                    catch (e) {
                        await message(e as string, { title: 'Error', type: 'error' });
                    }

                    refresh();
                    refetchContent();
                }

                setEdit(!edit());
                setError("");
            }}>
                <div class="flex flex-row items-center m-3 gap-3">
                    <div class="shadow-md rounded-md bg-[#98989D] min-w-16 w-16 min-h-16 h-16 flex items-center justify-center">
                        <GetSVG name={KindSVG(record().category)} class="w-8 h-8 fill-white"/>
                    </div>
                    <div class="flex flex-col w-full">
                        <input type="text" class="bg-inherit border-none truncate text-left text-[18px] w-full invalid:text-red-500 read-only:cursor-pointer read-only:hover:text-[#0064E1] read-only:select-none" value={record().title} placeholder="Title" readOnly={!edit()} required
                               onChange={(event) => {
                                   record().title = event.target.value;
                               }}
                               onInvalid={(event) => {
                                   event.preventDefault();
                                   setError("Title can not be empty");
                               }}
                               onClick={async () => {
                                   if (!edit()) await copyTextToClipboard(record().title)
                               }}>
                        </input>
                        <input type="text" class="bg-inherit border-none truncate text-left text-[14px] w-full invalid:text-red-500 read-only:cursor-pointer read-only:hover:text-[#0064E1] read-only:select-none" value={record().subtitle} placeholder="Subtitle" readOnly={!edit()} required
                               onChange={(event) => {
                                   record().subtitle = event.target.value;
                               }}
                               onInvalid={(event) => {
                                   event.preventDefault();
                                   setError("Subtitle can not be empty");
                               }}
                               onClick={async () => {
                                   if (!edit()) await copyTextToClipboard(record().subtitle)
                               }}>
                        </input>
                        <div class="flex flex-row justify-between items-center w-full">
                            <div class="flex flex-col">
                                <p class="text-[12px] text-[#828282] dark:text-[#9F9F9F] truncate">Created: {new Date(record().created as string).toLocaleString()}</p>
                                <p class="text-[12px] text-[#828282] dark:text-[#9F9F9F] truncate">Last modified: {new Date(record().last_modified as string).toLocaleString()}</p>
                            </div>
                            <button type="submit" title={(edit() ? "Save": "Edit")}>
                                <GetSVG name={(edit() ? "floppy-disk": "pen-to-square")} class="w-5 cursor-pointer hover:fill-[#0064E1]" />
                            </button>
                        </div>
                    </div>
                </div>
                <p class="flex justify-center text-[14px] truncate text-red-500 w-full" hidden={error().length == 0}>{error()}</p>
                <div class="w-full h-px bg-[#E7E7E7] dark:bg-[#3A3A3A]"></div>
                <div class="flex flex-col justify-between mx-3 my-1 transition ease-in duration-700">
                    <For each={allContent()}>{(content, index) =>
                        <>
                            <div class="flex flex-row gap-3">
                                <Show when={edit()}>
                                    <div class="flex flex-col gap-2 justify-evenly">
                                        <Show when={(index() > 0)}>
                                            <button title="Move up" onClick={(event) => {
                                                event.preventDefault();
                                                if(index() === 0) return;
                                                allContent()![index() - 1].position = index() + 1;
                                                content.position = index() - 1;
                                                let temp: Content[] = Object.assign([], allContent() as Content[]);
                                                temp[index()] = allContent()![index() - 1];
                                                temp[index() - 1] = allContent()![index()];
                                                newContent(temp);
                                            }}>
                                                <GetSVG name="arrow-up" class="w-3 hover:fill-[#0064E1]"/>
                                            </button>
                                        </Show>
                                        <Show when={!content.required}>
                                            <button title={"Delete" + ((content.label) ? " " + content.label : "")} onClick={async (event) => {
                                                event.preventDefault();
                                                const confirmed = await confirm("Are you sure you want to delete this content?",{title: "Delete content", type: "warning"});
                                                if(!confirmed) {
                                                    return;
                                                }
                                                if(content.id !== undefined && content.id !== 0){
                                                    await invoke("delete_content",{content: content});
                                                }

                                                let temp: Content[] = Object.assign([], allContent() as Content[]);
                                                temp.splice(index(), 1);
                                                temp.forEach((item, index) => item.position = index);
                                                newContent(temp);
                                            }}>
                                                <GetSVG name="trash" class="w-3 hover:fill-red-500"/>
                                            </button>
                                        </Show>
                                        <Show when={(index() < (allContent()?.length as number - 1))}>
                                            <button title="Move down" onClick={(event) => {
                                                event.preventDefault();
                                                if(index() === (allContent()?.length as number - 1)) return;
                                                allContent()![index() + 1].position = index() + 1;
                                                content.position = index() + 1;
                                                let temp: Content[] = Object.assign([], allContent() as Content[]);
                                                temp[index()] = allContent()![index() + 1];
                                                temp[index() + 1] = allContent()![index()];
                                                newContent(temp);
                                            }}>
                                                <GetSVG name="arrow-down" class="w-3 hover:fill-[#0064E1]"/>
                                            </button>
                                        </Show>
                                    </div>
                                </Show>
                                <ContentValue content={content}/>
                            </div>
                            <Show when={(index() < (allContent()?.length as number - 1)) || edit()}>
                                <div class="my-1 w-full h-px bg-[#E7E7E7] dark:bg-[#3A3A3A]"></div>
                            </Show>
                        </>
                    }</For>
                    <Show when={edit()}>
                        <div class="flex justify-center w-full">
                            <button class="py-1" title="Add new content" onClick={async (event) => {
                                event.preventDefault();
                                function addContent(kind: string) {
                                    let temp: Content[] = Object.assign([], allContent() as Content[]);
                                    temp.push(new Content("", allContent()?.length as number + 1, false, kind, ""));
                                    newContent(temp);
                                }
                                await showMenu({
                                    items: [
                                        {
                                            label: "Number",
                                            disabled: false,
                                            event: () => addContent("Number"),
                                        },
                                        {
                                            label: "Text",
                                            disabled: false,
                                            event: () => addContent("Text"),
                                        },
                                        {
                                            label: "Long text",
                                            disabled: false,
                                            event: () => addContent("LongText"),
                                        },
                                        {
                                            label: "Sensitive text",
                                            disabled: false,
                                            event: () => addContent("SensitiveText"),
                                        },
                                        {
                                            label: "Date",
                                            disabled: false,
                                            event: () => addContent("Date"),
                                        },
                                        {
                                            label: "Password",
                                            disabled: false,
                                            event: () => addContent("Password"),
                                        },
                                        {
                                            label: "TOTP secret",
                                            disabled: false,
                                            event: () => addContent("TOTPSecret"),
                                        },
                                        {
                                            label: "URL",
                                            disabled: false,
                                            event: () => addContent("Url"),
                                        },
                                        {
                                            label: "Email",
                                            disabled: false,
                                            event: () => addContent("Email"),
                                        },
                                        {
                                            label: "Phone number",
                                            disabled: false,
                                            event: () => addContent("PhoneNumber"),
                                        },
                                        {
                                            label: "Bank card number",
                                            disabled: false,
                                            event: () => addContent("BankCardNumber"),
                                        }]
                                });
                            }
                            }>
                                <GetSVG name="circle-plus" class="w-5 h-5 hover:fill-[#0064E1]" />
                            </button>
                        </div>
                    </Show>
                </div>
            </form>
        </div>
    )
}

/**
 * Content value component for the record detail
 * @param content - The content to be displayed.
 * @return {JSX.Element} - Div containing the content value.
 */
function ContentValue({content}: {content: Content}): JSX.Element {
    const [edit, _] = editSignal;
    const [error, setError] = createSignal("");
    const [additional, setAdditional] = createSignal<number | string | [string,number]>("");
    const [visibility, setVisibility] = createSignal(false);

    const [value] = createResource(
        () => [edit(), visibility()] as const,
        async ([edit, visible]) => {
            if((content.kind === "SensitiveText" || content.kind === "Password" || content.kind === "BankCardNumber" || content.kind === "TOTPSecret")){
                if(content.id === undefined || content.id === 0){
                    content.value = "";
                }
                else if (edit || visible){
                    try {
                        content.value = await invoke("get_content_value",{id: content.id as number}) as string;
                        if(content.kind === "Password"){
                            setAdditional(await invoke("password_strength",{password: content.value}) as number);
                        }
                    }
                    catch (e) {
                        await message(e as string, { title: 'Error', type: 'error' });
                    }
                }
                else {
                    content.value = "*******************" as string;
                }
            }
            else if ((content.kind === "Date") && (content.id === undefined || content.id === 0)) {
                content.value = (new Date()).toISOString().slice(0, 10);
            }
            return content.value
    });

    let placeholder = "Enter value";

    switch (content.kind) {
        case "Number": {
            placeholder = "Number";
            break;
        }
        case "Text": {
            placeholder = "Text";
            break;
        }
        case "LongText": {
            placeholder = "Long text";
            break;
        }
        case "SensitiveText": {
            placeholder = "Sensitive text";
            break;
        }
        case "Password": {
            placeholder = "Password";
            if(content.id !== undefined && content.id !== 0) {
                invoke("check_password",{id: content.id as number}).then((value) => {
                    if (value === "Common"){
                        setError("This password is too common.");
                    }
                    else if (value === "Exposed"){
                        setError("This password has been exposed in a data breach.");
                    }
                    else{
                        setError("");
                    }
                }).catch(_ => _);
            }
            break;
        }
        case "TOTPSecret": {
            placeholder = "Totp secret";
            if (content.id !== undefined && content.id !== 0) {
                invoke("get_totp_code",{id: content.id as number}).then((value) => setAdditional(value as [string, number]));
                let intervalId = setInterval(async () => setAdditional(await invoke("get_totp_code",{id: content.id as number})), 1000);

                onCleanup(() => {
                    if (intervalId) {
                        clearInterval(intervalId);
                    }
                });
            }
            break;
        }
        case "Url": {
            placeholder = "Url, e.g. https://example.com";
            break;
        }
        case "Email": {
            placeholder = "Email";
            break;
        }
        case "PhoneNumber": {
            placeholder = "International phone number, e.g. +420xxxxxxxxx";
            break;
        }
        case "BankCardNumber": {
            placeholder = "Enter bank card number";
            if (content.id !== undefined && content.id !== 0) {
                invoke("card_type",{id: content.id as number}).then((value) => setAdditional(value as string)).catch(_ => _);
            }
            break;
        }
    }

    async function copyValueToClipboard(id: number, label: string) {
        let items: Item[] = [{
            label: "Copy value of " + label,
            disabled: false,
            event: async (event) => await invoke("copy_value_to_clipboard",{id: event?.payload.id as number}).catch(async (e) => await message(e as string, { title: 'Error', type: 'error' })),
            payload: {id: id},
        }]
        if(content.kind === "SensitiveText" || content.kind === "Password" || content.kind === "BankCardNumber") {
            items.push({
                is_separator: true,
            })
            items.push({
                label: (visibility() ? "Hide" : "Reveal") + " value of " + label,
                disabled: false,
                event: () => setVisibility(!visibility()),
                checked: visibility(),
            });
        }

        await showMenu({
            items: items
        });
    }

    return (
        <div class="w-full">
            <input type="text" class="bg-inherit border-none truncate text-left w-full invalid:text-red-500 read-only:cursor-pointer read-only:hover:text-[#0064E1] read-only:select-none" value={content.label} placeholder="Label" readOnly={!edit()} required
                    onChange={(event) => {
                        content.label = event.target.value;
                    }}
                    onInvalid={(event) => {
                        event.preventDefault();
                        setError("Label can not be empty");
                    }}
                    onClick={async () => {
                        if (!edit()) await copyTextToClipboard(content.label)
                    }}>
            </input>
            <div class={`flex flex-row w-full ${edit() ? "": "hover:text-[#0064E1] cursor-pointer"}`} onClick={async () => {
                if (!edit()) await copyValueToClipboard(content.id as number,content.label)
            }}>
                <Show when={content.kind == "BankCardNumber"}>
                    <div>{additional()}</div>
                </Show>
                <div class="flex flex-col w-full">
                    <Switch fallback={
                        <>
                            <input type={(content.kind === "Date") ? "date" : "text"}
                                 class="bg-inherit border-none truncate justify-end text-right w-full invalid:text-red-500 read-only:pointer-events-none read-only:select-none"
                                 value={value.latest} placeholder={placeholder} readOnly={!edit()} required
                                 onInput={async (event) => {
                                     content.value = event.target.value
                                     let [valid, error]: [boolean, string?] = await invoke("valid", {
                                         kind: content.kind,
                                         value: content.value
                                     });
                                     if (valid) {
                                         event.target.setCustomValidity("");
                                         setError("");
                                         if (content.kind == "BankCardNumber") {
                                             setAdditional(await invoke("card_type", {id: content.id}) as string);
                                         }
                                         if (content.kind == "Password") {
                                             setAdditional(await invoke("password_strength", {password: content.value}) as number);
                                         }
                                     } else if (error) {
                                         event.target.setCustomValidity(error);
                                     } else {
                                         event.target.setCustomValidity("Invalid value");
                                     }
                                 }}
                                 onInvalid={(event) => {
                                     event.preventDefault();
                                     setError(event.currentTarget.validationMessage);
                                 }}>
                            </input>
                            <Show when={content.kind == "Password" && edit() && additional()}>
                                <PasswordStrengthIndicator strength={additional as () => number} />
                            </Show>
                        </>
                    }>
                        <Match when={content.kind == "LongText"}>
                            <textarea class="bg-inherit border-none justify-end w-full min-h-52 read-only:cursor-pointer read-only:select-none resize-y overflow-scroll" placeholder={placeholder} readOnly={!edit()}
                                      onInput={async (event) => {
                                          content.value = event.target.value;
                                      }}>
                                {value.latest}
                            </textarea>
                        </Match>
                        <Match when={content.kind == "TOTPSecret" && !edit()}>
                            <div class="flex flex-row w-full">
                                <div class="bg-inherit border-none truncate justify-end text-right grow invalid:text-red-500 read-only:pointer-events-none read-only:select-none">{(additional() as [string, number])[0]}</div>
                                <div class="radial-progress ml-2 text-[70%]" style={{ "--value": `${(((additional() as [string, number])[1] as number)/30 * 100)}`, "--size": "2rem", "--thickness": "3px" }} role="progressbar">{(additional() as [string, number])[1]}</div>
                            </div>
                        </Match>
                    </Switch>
                </div>
            </div>
            <p class="flex justify-center text-[14px] truncate text-red-500 w-full" hidden={error().length == 0}>{error()}</p>
        </div>
    )
}