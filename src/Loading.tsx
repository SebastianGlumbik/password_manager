import {JSX} from "solid-js";

/**
 * Loading component
 * Has a loading spinner and a text saying "Loading, please wait"
 * @return {JSX.Element} - Div containing the loading component.
 */
export default function Loading(): JSX.Element {
  return (
      <div class="h-full flex flex-col items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-10 animate-spin" ><path d="M304 48a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zm0 416a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM48 304a48 48 0 1 0 0-96 48 48 0 1 0 0 96zm464-48a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM142.9 437A48 48 0 1 0 75 369.1 48 48 0 1 0 142.9 437zm0-294.2A48 48 0 1 0 75 75a48 48 0 1 0 67.9 67.9zM369.1 437A48 48 0 1 0 437 369.1 48 48 0 1 0 369.1 437z"/></svg>
          <p class="text-2xl">Loading, please wait</p>
      </div>
  );
}