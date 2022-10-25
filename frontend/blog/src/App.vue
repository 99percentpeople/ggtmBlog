<template>
  <html-header></html-header>

  <n-config-provider
    :breakpoints="reactiveSize"
    :locale="zhCN"
    :theme="isDark ? darkTheme : null"
    :theme-overrides="isDark ? darkThemeOverrides : lightThemeOverrides"
    :date-locale="dateZhCN"
    style="height: inherit"
  >
    <n-message-provider>
      <navigate />
    </n-message-provider>
    <n-global-style />
  </n-config-provider>
</template>

<script setup lang="ts">
import { zhCN, dateZhCN, darkTheme, GlobalThemeOverrides } from "naive-ui";
import { useSetting } from "@/stores";
const setting = useSetting();
let { reactiveSize, isDark } = storeToRefs(setting);

const lightThemeOverrides: GlobalThemeOverrides = {
  common: {
    bodyColor: "rgb(243,243,243)",
  },
  Button: {},
};
const darkThemeOverrides: GlobalThemeOverrides = {};
</script>
