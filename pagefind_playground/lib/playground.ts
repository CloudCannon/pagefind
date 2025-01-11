import PlaygroundSvelte from "./Playground.svelte";
import { mount } from "svelte";

declare var pagefind_version: string;

document.addEventListener("DOMContentLoaded", () => {
  const dom = document.querySelector("#playground")!;
  mount(PlaygroundSvelte, {
    target: dom,
    props: { pagefindVersion: pagefind_version },
  });
});
