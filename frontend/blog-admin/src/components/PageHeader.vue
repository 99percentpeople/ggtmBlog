<template>
    <n-layout-header class="header" inverted bordered style="padding: 12px">
        <n-config-provider :theme-overrides="isDark ? darkThemeOverrides : lightThemeOverrides">
            <n-space justify="space-between">
                <n-el>
                        <h2><a :href="origin">ggtmBlog</a></h2>
                </n-el>
                <user-avatar></user-avatar>
            </n-space>
        </n-config-provider>
    </n-layout-header>
</template>

<script setup lang="tsx">
import { useSetting } from "@/stores";
import { GlobalThemeOverrides, darkTheme, NLayoutHeader, NSpace, NEl } from "naive-ui";
const setting = useSetting();
const isDark = false;
const origin = location.origin;
const lightThemeOverrides: GlobalThemeOverrides = {
    common: {
        textColor1: darkTheme.common?.textColor1,
        // textColor2: darkTheme.common?.textColor2,
        textColor3: darkTheme.common?.textColor3,
        primaryColor: darkTheme.common?.primaryColor,
        primaryColorHover: darkTheme.common?.primaryColorHover,
    },
    Input: {
        textColor: darkTheme.common?.textColor2,
        color: darkTheme.Input.common?.bodyColor,
        colorFocus: darkTheme.Input.common?.inputColor,
    },
    Button: {
        textColor: darkTheme.Button.common?.textColor2,
    },
    Switch: {
        railColor: darkTheme.Switch.common?.railColor,
    },
    // ...
};
const darkThemeOverrides: GlobalThemeOverrides = {
    // ...
};
</script>
<style scoped lang="scss">
.header-layout {
    display: flex;
    align-items: flex-start;
    padding-right: 24px;
    & > * {
        padding: 0 12px;
    }
}
</style>
