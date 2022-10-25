<template>
    <n-space>
        <n-dropdown trigger="hover" :options="options" @select="handleSelect">
            <n-button strong secondary type="info">操作</n-button>
        </n-dropdown>
        <n-popover trigger="hover">
            <template #trigger>
                <n-avatar round :src="avatarLink">{{ loginStatus.userInfo.username }}</n-avatar>
            </template>
            <n-space vertical>
                <n-space align="center">
                    <n-avatar round size="large" :src="avatarLink"></n-avatar>
                    <n-text>12篇文章</n-text>
                </n-space>
                <n-space justify="center">
                    <n-text tag="a" href="#">进入主页</n-text>
                </n-space>
            </n-space>
        </n-popover>
    </n-space>
</template>

<script setup lang="tsx">
import { DropdownOption, NButton, NIcon, NText, useMessage } from "naive-ui"
import { Person24Regular, TabArrowLeft24Regular } from "@vicons/fluent"
import { useLoginStatus } from "@/stores";
import axios from "axios";
const loginStatus = useLoginStatus();
const router = useRouter();
const avatarLink = ""
const message = useMessage();
const options = [] as DropdownOption[]

if (loginStatus.loginStatus) {
    options.push({
        label: "注销",
        key: 'logout',
        icon: () => <NIcon component={Person24Regular} />
    })
} else {
    options.push({
        label: "登录",
        key: 'login',
        icon: () => <NIcon component={TabArrowLeft24Regular} />
    })
}

function handleSelect(key: string | number) {
    switch (key) {
        case 'login': {
            router.replace({ name: "login" })
            break
        }
        case "logout": {
            axios.post("/api/logout").then(res => {
                message.info(res.data.msg)
                loginStatus.loginStatus = false
                router.replace({ name: "login" })
            }).catch(err => {
                message.error(err.response.data)
            })
            break
        }
    }
}

</script>

<style lang="scss">
.avatar {
    display: flex;
    align-items: center;
    flex-wrap: nowrap;
    & > :first-child {
        padding-top: 0;
        padding-bottom: 0;
        margin: 0 12px;
    }
}
</style>