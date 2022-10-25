<template>
    <n-layout class="navigate-wrapper" content-style="height:inherit;display:flex;flex-direction:column;" style="min-width: 1080px">
        <page-header></page-header>
        <n-layout :style="{ '--footerHeightString': footerHeightString }" has-sider>
            <n-layout-sider bordered show-trigger="bar" collapse-mode="width" :collapsed-width="64" :width="240" :native-scrollbar="false" inverted>
                <n-menu inverted :collapsed-width="64" :collapsed-icon-size="22" :options="menuOptions" :value="$route.name?.toString()" />
            </n-layout-sider>
            <n-layout-content class="content-wrapper" embedded>
                <div class="content">
                    <slot></slot>
                </div>
                <page-footer ref="footer"></page-footer>
                <div class="box"></div>
            </n-layout-content>
        </n-layout>
    </n-layout>
</template>

<script setup lang="tsx">
import { Home16Regular, AddSquareMultiple20Regular, Tag24Regular, AppsListDetail24Regular, ImageMultiple24Regular } from "@vicons/fluent";
import { RouterLink } from "vue-router";
import { useElementSize } from "@vueuse/core";
const menuOptions = [
    {
        label: () => <RouterLink to={{ name: "home" }}>首页</RouterLink>,
        key: "home",
        icon: () => <Home16Regular />,
    },
    {
        label: () => <RouterLink to={{ name: "sort" }}>分类</RouterLink>,
        key: "sort",
        icon: () => <AppsListDetail24Regular />,
    },
    {
        label: () => <RouterLink to={{ name: "tag" }}>标签</RouterLink>,
        key: "tag",
        icon: () => <Tag24Regular />,
    },
    {
        label: () => <RouterLink to={{ name: "publish" }}>发布</RouterLink>,
        key: "publish",
        icon: () => <AddSquareMultiple20Regular />,
    },
    {
        label: () => <RouterLink to={{ name: "file" }}>文件</RouterLink>,
        key: "file",
        icon: () => <ImageMultiple24Regular />,
    },
];
const footer = ref<HTMLElement | null>(null);
const { height } = useElementSize(footer);

const footerHeightString = computed(() => {
    return `${height.value}px`;
});
</script>

<style scoped lang="scss">
.navigate-wrapper {
    height: inherit;
}
.content-wrapper {
    flex: 1 1 auto;
}
.content {
    padding: 12px;
    min-height: calc(100% - var(--footerHeightString));
}
</style>
