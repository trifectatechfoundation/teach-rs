
<!--
This theme adds a link to the Rust playground to code snippets
  Usage:
```md
---
layout: playground
---
```rust
// Your code here
```
-->

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
  },
});

// TODO: make sure this runs exactly once for every code block
// TODO: reimplement this in Vue and make it work with hmr
const codeElement = document.querySelector("pre.slidev-code code");
if (codeElement) {
    const lineElements = Array.from(codeElement.querySelectorAll("span.line"));
    let code = lineElements.map(line => {
        const tokens = line.querySelectorAll("span");
        return line.textContent;
    }).join("\n");

    const playgroundUrl = "https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=" + encodeURIComponent(code);

    const playgroundElement = document.createElement("a");
    playgroundElement.setAttribute("class", "playground");
    playgroundElement.innerHTML = "▶ Playground";
    playgroundElement.setAttribute("target", "_blank");

    playgroundElement.setAttribute("href", playgroundUrl);
    const container = codeElement.closest(".slidev-code-wrapper");
    container.insertBefore(playgroundElement, container.firstChild);
}
</script>

<template>
  <div class="slidev-layout ">
    <div :class="props.class">
      <slot />
    </div>
  </div>
</template>
