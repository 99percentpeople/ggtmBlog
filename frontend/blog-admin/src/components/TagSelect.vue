<template>
    <n-select
        :value="selectValue"
        @update:value="hadleSelect"
        :consistent-menu-width="false"
        filterable
        placeholder="搜索分类"
        :options="optionsRef"
        :loading="loadingRef"
        clearable
        multiple
        remote
        @search="handleSearch"
        @scroll="handleScroll"
        @clear="handleClear"
    ></n-select>
</template>

<script setup lang="ts">
import { getTagsList } from "@/apis";
import { Tag, TagQuery } from "@/models";
import { SelectMixedOption } from "naive-ui/lib/select/src/interface";
/////////////////////////////////////////////////////////////////////
//                            分类选择框                             //
/////////////////////////////////////////////////////////////////////
const loadingRef = ref(true);
const optionsRef = ref([] as SelectMixedOption[]);
const props = defineProps<{
    modelValue?: string[] | number[];
}>();
const tagQuery = reactive<TagQuery>({
    name: undefined,
    published: undefined,
});
const page = ref(1);
const pageCount = ref(1);
onMounted(() => {
    createTagList();
});

const emits = defineEmits<{
    (event: "update:modelValue", value: string[] | number[] | undefined): void;
}>();
const selectValue = computed({
    get: () => props.modelValue,
    set: (value) => emits("update:modelValue", value),
});
function hadleSelect(value: string[] | number[]) {
    selectValue.value = value;
}
function createTagList() {
    loadingRef.value = true;
    getTagsList(10, 1, tagQuery)
        .then(({ data }) => {
            optionsRef.value = data.list.map((item: Tag) => ({ label: item.name, value: item.id }));
            pageCount.value = data.pages;
            page.value = 1;
        })
        .finally(() => {
            loadingRef.value = false;
        });
}

function handleClear() {
    return createTagList();
}
function handleSearch(query: string) {
    if (!query.length) {
        return createTagList();
    } else {
        loadingRef.value = true;
        return getTagsList(10, 1, { name: query })
            .then(({ data }) => {
                optionsRef.value = data.list.map((item: Tag) => ({ label: item.name, value: item.id }));
            })
            .finally(() => {
                loadingRef.value = false;
            });
    }
}
function handleScroll(e: Event) {
    const currentTarget = e.currentTarget as HTMLElement;
    if (currentTarget.scrollTop + currentTarget.offsetHeight >= currentTarget.scrollHeight - 1 && page.value < pageCount.value && !loadingRef.value) {
        loadingRef.value = true;
        getTagsList(10, page.value + 1, tagQuery)
            .then(({ data }) => {
                // message.info(JSON.stringify(data.list))
                optionsRef.value = optionsRef.value.concat(data.list.map((item: Tag) => ({ label: item.name, value: item.id })));
                page.value += 1;
            })
            .finally(() => {
                loadingRef.value = false;
            });
    }
}
</script>
