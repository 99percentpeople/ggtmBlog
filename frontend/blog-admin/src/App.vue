<template>
    <div class="app">
        <n-config-provider
            :breakpoints="reactive"
            :locale="zhCN"
            :date-locale="dateZhCN"
            style="height:inherit"
        >
            <n-message-provider>
                <router-view #="{ Component, route }">
                    <transition :name="route.meta.transition">
                        <component :is="Component" />
                    </transition>
                </router-view>
            </n-message-provider>
        </n-config-provider>
    </div>
</template>

<script setup lang="ts">

import { zhCN, dateZhCN } from 'naive-ui'
import { useSetting } from '@/stores';
import { storeToRefs } from 'pinia';
const setting = useSetting();
let { reactive } = storeToRefs(setting)


</script>

<style lang="scss">
@import "@/styles/router-translations";
.app {
    height: 100vh;
}
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}
a {
    text-decoration: none;

    &:hover {
        text-decoration: underline;
    }
}
</style>