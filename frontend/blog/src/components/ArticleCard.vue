<template>
    <n-el tag="div">
        <n-card :bordered="true" hoverable v-if="loading">
            <template #header>
                <n-space>
                    <n-skeleton height="29px" width="180px" />
                    <n-skeleton height="29px" width="42px" />
                </n-space>
            </template>
            <n-grid x-gap="12" y-gap="24" cols="1 s:2 m:5" item-responsive responsive="screen">
                <n-gi :span="3"> <n-skeleton text :repeat="2" /> <n-skeleton text style="width: 60%" /> </n-gi>
                <n-gi :span="2">
                    <n-space justify="center">
                        <n-skeleton height="200px" width="300px" />
                    </n-space>
                </n-gi>
            </n-grid>
            <template #footer>
                <n-space align="center" justify="space-between">
                    <n-space align="center">
                        <n-skeleton height="34px" circle />
                        <n-skeleton height="22px" width="136px" />
                        <n-skeleton height="22px" width="46px" />
                    </n-space>
                    <n-space align="center" :size="6">
                        <n-skeleton height="22px" width="36px" />
                        <n-skeleton height="22px" width="36px" />
                        <n-skeleton height="22px" width="36px" />
                        <n-skeleton height="22px" width="36px" />
                    </n-space>
                </n-space>
            </template>
        </n-card>
        <n-card :bordered="true" hoverable v-else>
            <template #header>
                <n-space>
                    <router-link :to="{ name: 'blog', params: { id: blogItem?.id } }">
                        <n-text tag="a">{{ blogItem?.title }}</n-text>
                    </router-link>
                    <n-tag>{{
                        blogItem?.flag == "Original" ? "原创" : blogItem?.flag == "Reprint" ? "转载" : "翻译"
                    }}</n-tag>
                </n-space>
            </template>
            <template #footer>
                <n-space align="center" justify="space-between">
                    <n-space align="center">
                        <avatar :user-info="blogItem?.user">
                            <n-text>{{ blogItem?.user.nickname }}</n-text>
                        </avatar>
                        <n-tag :bordered="false" size="small">
                            <template #avatar>
                                <n-icon>
                                    <CalendarDay24Regular />
                                </n-icon>
                            </template>
                            <n-text>{{ blogItem?.create_time?.toLocaleString() }}</n-text>
                        </n-tag>

                        <n-tag :bordered="false" size="small">
                            <template #avatar>
                                <n-icon>
                                    <Eye24Regular />
                                </n-icon>
                            </template>
                            <n-text>{{ blogItem?.views }}</n-text>
                        </n-tag>
                    </n-space>
                    <n-space align="center" justify="end" :size="6">
                        <n-tag type="info" size="small">{{ blogItem?.sort?.name }}</n-tag>
                        <n-tag v-for="tag in blogItem?.tags" size="small">{{ tag.name }}</n-tag>
                    </n-space>
                </n-space>
            </template>
            <n-grid x-gap="12" y-gap="24" cols="1 s:2 m:5" item-responsive responsive="screen">
                <n-gi :span="blogItem?.cover ? 3 : 5" v-if="blogItem?.summary">
                    <n-ellipsis :line-clamp="3" :tooltip="false">
                        <n-text depth="3">{{ blogItem.summary }}</n-text>
                    </n-ellipsis>
                </n-gi>
                <n-gi :span="blogItem?.summary ? 2 : 5" v-if="blogItem?.cover">
                    <n-space justify="center">
                        <n-image
                            :src="blogItem?.cover"
                            :img-props="{
                                style: `display:inline-block;max-height:250px;max-width:100%;border-radius: 6px;`,
                            }"
                            object-fit="cover"
                        />
                    </n-space>
                </n-gi>
            </n-grid>
        </n-card>
    </n-el>
</template>

<script setup lang="ts">
import { Eye24Regular, CalendarDay24Regular } from "@vicons/fluent";
import { BlogDetailModel } from "common/models";
const router = useRouter();
const props = defineProps<{
    loading?: boolean;
    blogItem?: BlogDetailModel;
}>();
</script>

<style scoped lang="scss"></style>
