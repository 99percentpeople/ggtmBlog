<template>
    <n-layout embedded class="page-wrapper" content-style="display:flex;flex-direction:column;align-items:center;justify-content:center;padding:12px;">
        <n-h1>Blog-Admin</n-h1>

        <n-card style="max-width: 720px">
            <template #header>
                <div>登录</div>
            </template>
            <n-form ref="formRef" :model="userFormValue" label-align="left" label-width="auto">
                <n-form-item label="用户名" path="name">
                    <n-input v-model:value="userFormValue.username" placeholder="请输入用户名" />
                </n-form-item>
                <n-form-item label="密码" path="password">
                    <n-input v-model:value="userFormValue.password" type="password" placeholder="请输入密码" />
                </n-form-item>
            </n-form>
            <template #action>
                <n-space justify="space-between" align="center">
                    <n-space>
                        <n-button #icon circle size="tiny">
                            <Question16Regular />
                        </n-button>
                    </n-space>
                    <n-space>
                        <n-button @click="$router.back">返回</n-button>
                        <n-button @click="onLoginClick" type="primary">登录</n-button>
                    </n-space>
                </n-space>
            </template>
        </n-card>
    </n-layout>
</template>

<script lang="ts" setup>
import { FormInst, MessageReactive, NInput, useMessage } from "naive-ui";
import { Question16Regular } from "@vicons/fluent";
import Axios from "axios";
import { useLoginStatus } from "@/stores";
import { UserInfo } from "@/models";

const vertifyInput = ref<HTMLElement | null>(null);
const router = useRouter();
const loginStatus = useLoginStatus();
const message = useMessage();
const userFormValue = ref({
    password: "",
    username: "",
});

const formRef = ref<FormInst | null>(null);
function onLoginClick(e: MouseEvent) {
    e.preventDefault();
    formRef.value?.validate((errors) => {
        if (!errors) {
            Axios.post("/api/login", userFormValue.value, {})
                .then((res) => {
                    message.info(res.data.msg);
                    if (res.data.data) {
                        loginStatus.userInfo = {
                            ...res.data.data,
                            createTime: new Date(res.data.data.create_time),
                        } as UserInfo;
                    }
                    loginStatus.loginStatus = true;
                    router.push({ name: "admin" });
                })
                .catch((err) => {
                    message.error(err.response.data);
                });
        } else {
            message.error(errors.toString());
        }
    });
}

function onVerify() {
    if (loginStatus.userInfo.access_level) {
        Axios.post(`/api/verify/${loginStatus.userInfo.access_level}`)
            .then((res) => {
                message.info(JSON.stringify(res.data));
            })
            .catch((err) => {
                message.info(JSON.stringify(err.response.data));
            });
    } else {
    }
}

</script>

<style scpoed lang="scss">
.page-wrapper {
    height: inherit;
}
</style>
