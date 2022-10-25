<template>
    <n-select
        :value="selectValue"
        @update:value="hadleSelect"
        filterable
        placeholder="搜索分类"
        :options="optionsRef"
        :loading="loadingRef"
        clearable
        remote
        @search="handleSearch"
        @scroll="handleScroll"
        @clear="handleClear"
    ></n-select>
</template>

<script setup lang="ts">
import { getSortList } from "@/apis";
import { SortQuery } from "@/models";
import { Sort } from "@/models";
import { SelectMixedOption } from "naive-ui/lib/select/src/interface";
/////////////////////////////////////////////////////////////////////
//                            分类选择框                             //
/////////////////////////////////////////////////////////////////////
const loadingRef = ref(true);
const optionsRef = ref([] as SelectMixedOption[]);
const props = defineProps<{
    modelValue?: string | number | null;
}>();
const emits = defineEmits<{
    (event: "update:modelValue", value: string | number | null | undefined): void;
}>();
const selectValue = computed({
    get: () => props.modelValue,
    set: (value) => emits("update:modelValue", value),
});

function hadleSelect(value: string | number | null) {
    selectValue.value = value;
}
const page = ref(1);
const pageCount = ref(1);
onMounted(() => {
    createSortList();
});
const sortQuery = reactive<SortQuery>({
    name: undefined,
    published: undefined,
});

function createSortList() {
    loadingRef.value = true;
    getSortList(10, 1, sortQuery)
        .then(({ data }) => {
            optionsRef.value = data.list.map((item: Sort) => ({ label: item.name, value: item.id }));
            pageCount.value = data.pages;
            page.value = 1;
        })
        .finally(() => {
            loadingRef.value = false;
        });
}

function handleClear() {
    return createSortList();
}
function handleSearch(query: string) {
    if (!query.length) {
        return createSortList();
    } else {
        loadingRef.value = true;
        return getSortList(10, 1, { name: query })
            .then(({ data }) => {
                optionsRef.value = data.list.map((item: Sort) => ({ label: item.name, value: item.id }));
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
        getSortList(10, page.value + 1, sortQuery)
            .then(({ data }) => {
                // message.info(JSON.stringify(data.list))
                optionsRef.value = optionsRef.value.concat(data.list.map((item: Sort) => ({ label: item.name, value: item.id })));
                page.value += 1;
            })
            .finally(() => {
                loadingRef.value = false;
            });
    }
}
</script>
