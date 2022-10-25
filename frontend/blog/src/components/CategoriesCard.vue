<template>
    <n-el>
        <n-list-item v-if="loading">
            <n-space vertical :size="6">
                <n-space>
                    <n-skeleton height="20px" width="110px" />
                    <n-skeleton height="20px" width="38px" />
                </n-space>
                <n-space :wrap="false" :size="6">
                    <n-skeleton height="22px" width="84px" />
                    <n-skeleton height="22px" width="40px" />
                </n-space>
            </n-space>
            <template #suffix>
                <n-skeleton height="22px" width="40px" />
            </template>
        </n-list-item>

        <n-list-item v-else>
            <n-space vertical :size="6">
                <n-space>
                    <router-link :to="{ name: 'blog', params: { id: item?.id } }">
                        <a>{{ item?.title }}</a>
                    </router-link>
                    <n-tag size="small">{{
                        item?.flag == "Original" ? "原创" : item?.flag == "Reprint" ? "转载" : "翻译"
                    }}</n-tag>
                </n-space>
                <n-space :wrap="false" :size="6">
                    <n-tag :bordered="false" size="small">
                        <template #avatar>
                            <n-icon>
                                <CalendarDay24Regular />
                            </n-icon>
                        </template>
                        <n-text>{{ item?.create_time?.toLocaleDateString() }}</n-text>
                    </n-tag>
                    <n-tag :bordered="false" size="small">
                        <template #avatar>
                            <n-icon>
                                <Eye24Regular />
                            </n-icon>
                        </template>
                        <n-text>{{ item?.views }}</n-text>
                    </n-tag>
                </n-space>
            </n-space>
            <template #suffix>
                <n-tag type="info" size="small">{{ item?.sort?.name }}</n-tag>
            </template>
        </n-list-item>
    </n-el>
</template>

<script setup lang="tsx">
import { BlogDetailModel } from "common/models";
import { CalendarDay24Regular, Eye24Regular } from "@vicons/fluent";
const props = defineProps<{
    loading?: boolean;
    item?: BlogDetailModel;
}>();
</script>

<style lang="scss"></style>
