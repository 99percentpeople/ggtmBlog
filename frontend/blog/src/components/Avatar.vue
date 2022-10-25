<template>
    <n-popover trigger="hover">
        <template #trigger>
            <n-avatar :size="size" round :src="userInfo?.avatar"></n-avatar>
        </template>
        <n-space vertical>
            <n-space align="center">
                <n-avatar round :src="userInfo?.avatar"></n-avatar>
                <n-text>{{ userInfo?.nickname ? userInfo.nickname : userInfo?.username }}</n-text>
            </n-space>
        </n-space>
    </n-popover>
</template>

<script setup lang="ts">
import { UserInfo } from "common/models";

const props = defineProps<{
    size?: "small" | "medium" | "large";
    userInfo?: UserInfo;
}>();
</script>
