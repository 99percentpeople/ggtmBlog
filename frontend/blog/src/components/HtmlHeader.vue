<template>
    <Head>
        <meta name="color-scheme" :content="isDark ? 'dark' : 'light'" />
        <title>{{ appTitle }}</title>
        <meta name="theme-color" :content="themeVars.invertedColor" />
    </Head>
</template>

<script setup lang="ts">
import { useSetting, useMemory } from "@/stores";
import { useThemeVars } from "naive-ui";
import { Head } from "@vueuse/head";
const themeVars = useThemeVars();
const setting = useSetting();
const memory = useMemory();
const { isDark } = storeToRefs(setting);
const { appTitle } = storeToRefs(memory);
</script>
