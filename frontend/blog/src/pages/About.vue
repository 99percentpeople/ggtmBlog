<template>
    <div class="view-wrapper">
        <n-grid x-gap="12" y-gap="12" cols="1 s:2" item-responsive responsive="screen" style="max-width: 1280px">
            <n-gi span="1">
                <n-card
                    size="small"
                    :segmented="{
                        content: true,
                    }"
                >
                    <template #header>
                        <n-h3 style="display: flex; align-items: center; margin: 0">关于</n-h3>
                    </template>
                    <n-space vertical align="center">
                        <n-image
                            src="/logo.jpg"
                            :img-props="{
                                style: `display:inline-block;max-height:200px;max-width:100%;border-radius:100%;`,
                            }"
                        ></n-image>
                    </n-space>
                </n-card>
            </n-gi>
            <n-gi span="1">
                <n-card
                    size="small"
                    :segmented="{
                        content: true,
                        footer: 'soft',
                    }"
                >
                    <template #header>
                        <n-h3 style="display: flex; align-items: center; margin: 0">关于</n-h3>
                    </template>
                    <n-text> 土木老哥，热爱编程 </n-text>
                    <template #footer>
                        <n-space>
                            <n-tag>编程</n-tag>
                            <n-tag>写作</n-tag>
                            <n-tag>博客</n-tag>
                            <n-tag>Web</n-tag>
                            <n-tag>Javascript</n-tag>
                            <n-tag>Typescript</n-tag>
                            <n-tag>Vue3</n-tag>
                            <n-tag>Rust</n-tag>
                            <n-tag>PostgreSQL</n-tag>
                        </n-space>
                    </template>
                    <template #action>
                        <contact-link />
                    </template>
                </n-card>
            </n-gi>
        </n-grid>
    </div>
</template>
<script setup lang="ts">
import { useSetting, useMemory } from "@/stores";
const memory = useMemory();
onMounted(() => {
    memory.setTitle("关于");
});
</script>
