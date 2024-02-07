import {createSignal} from "solid-js";

/**
 * Record model.
 */
export class Record {
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

/**
 * Content model.
 */
export class Content {
    id?: number;
    label: string;
    position: number;
    required: boolean;
    kind: string;
    value?: string;

    constructor(label: string, position: number, required: boolean, kind: string, value?: string, id?: number) {
        this.id = id;
        this.label = label;
        this.position = position;
        this.required = required;
        this.kind = kind;
        this.value = value;
    }
}

/**
 * Convert kind to SVG icon.
 * @param {string} kind - Kind of the record.
 * @return {string} - SVG icon name.
 */
export function KindSVG(kind: string): string {
    switch(kind) {
        case "Login": {
            return "globe";
        }
        case "Bank Card": {
            return "credit-card";
        }
        case "Note": {
            return "note-sticky";
        }
    }

    return "briefcase";
}

/**
 * Signal for editing a record.
 */
export const editSignal = createSignal(false);