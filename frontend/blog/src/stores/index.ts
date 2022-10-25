import { defineStore } from "pinia";
import styleReactive from "@/styles/_reactive.module.scss";
import { mapVal } from "@/utils";

export const useSetting = defineStore("setting", () => {
    const reactiveSize = ref(mapVal(styleReactive, parseInt));
    const isDark = useDark();
    const toggleDark = useToggle(isDark)
    return {
        isDark,
        toggleDark,
        reactiveSize,
    };
});

export const useLocalInfo = defineStore("localInfo", () => {
    const commentInfo = useLocalStorage("commentInfo", {
        remember: true,
        nickname: undefined,
        email: undefined,
        avatar: undefined,
    } as {
        remember: boolean;
        nickname?: string | null;
        email?: string | null;
        avatar?: string | null;
    });
    return {
        commentInfo,
    };
});

export const useMemory = defineStore("memory", () => {
    const appTitle = ref(import.meta.env.VITE_APP_TITLE);

    function setTitle(title?: string) {
        if (title) appTitle.value = `${title} - ${import.meta.env.VITE_APP_TITLE}`;
        else appTitle.value = import.meta.env.VITE_APP_TITLE;
    }
    return {
        appTitle,
        setTitle,
    };
});
