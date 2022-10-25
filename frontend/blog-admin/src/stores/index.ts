import { defineStore } from "pinia";
import styleReactive from "@/styles/_reactive.module.scss";
import { mapVal } from "@/utils";
import { ref } from "vue";
import {UserInfo} from "@/models"
export const useSetting = defineStore("setting", () => {
    const reactive = ref(mapVal(styleReactive, parseInt));

    return {
        reactive,
    };
});

export const useLoginStatus = defineStore("loginStatus", () => {
    const loginStatus = useLocalStorage("loginStatus", false);
    const userInfo = useLocalStorage("userInfo", {} as UserInfo);
    return {
        loginStatus,
        userInfo,
    };
});