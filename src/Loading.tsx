import {JSX} from "solid-js";
import GetSVG from "./GetSVG.tsx";

/**
 * Loading component
 * Has a loading spinner and a text saying "Loading...", aligned in the center of the screen.
 * @return {JSX.Element} - Div containing the loading component.
 */
export default function Loading(): JSX.Element {
  return (
      <div class="absolute bottom-0 h-full w-full flex flex-col items-center justify-center">
          <GetSVG name={"spinner"} class={"w-7 animate-spin"} />
          <p class="text-lg">Loading...</p>
      </div>
  );
}