<template>
    <n-config-provider :theme-overrides="themeOverrides">
        <n-layout-footer class="footer" inverted bordered>
            <n-divider style="margin: 0;padding:0">Copyright©2020-2022 Blog-Admin</n-divider>
            <n-space style="margin: 0" justify="center"></n-space>
        </n-layout-footer>
    </n-config-provider>
</template>

<script setup lang="ts">
import { GlobalThemeOverrides, darkTheme } from 'naive-ui';


const themeOverrides: GlobalThemeOverrides = {
    common: {
        textColor1: darkTheme.common?.textColor1,
        textColor2: darkTheme.common?.textColor2,
        textColor3: darkTheme.common?.textColor3,
        primaryColor: darkTheme.common?.primaryColor,
        primaryColorHover: darkTheme.common?.primaryColorHover,
    },
    Divider: {
        color: darkTheme.Divider.common?.borderColor,
    },
    Button: {
        textColorText: darkTheme.Button.common?.textColor2
    }

    // ...
}
</script>

<style scoped>
.footer {
    padding: 22px;
}
.sticky-footer {
    position: sticky;
    bottom: 0;
}
</style>