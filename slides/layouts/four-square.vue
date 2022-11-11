<!--
Usage:
```md
---
layout: four-square
---
```
-->

<script setup lang="ts">

const props = defineProps({
    class: {
        type: String,
    },
})
</script>

<template>
    <div class="slidev-layout ">
        <div :class="props.class">
            <slot/>
        </div>
        <div class="two-columns w-full h-full grid grid-cols-2 gap-2">
            <div class="col-top-left" :class="props.class">
                <slot name="topleft"/>
            </div>
            <div class="col-top-right" :class="props.class">
                <slot name="topright"/>
            </div>
            <div class="col-bottom-left" :class="props.class">
                <slot name="bottomleft"/>
            </div>
            <div class="col-bottom-right" :class="props.class">
                <slot name="bottomright"/>
            </div>
        </div>
    </div>
</template>
