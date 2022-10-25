<template>
    <div @click="handleOnClick" class="wrapper">
        <n-input
            v-if="isEdit"
            ref="inputRef"
            size="small"
            v-model:value="inputValue"
            @change="onChange"
            @blur="onBlur"
        ></n-input>
        <n-text v-else>{{ props.value }}</n-text>
    </div>
</template>

<script setup lang="ts">
import { NInput } from 'naive-ui'

const props = defineProps<{
    value: string | number | null,
    onUpdateValue: (arg0: string) => void
}>()
const isEdit = ref(false)
const inputRef = ref<typeof NInput | null>(null)
const inputValue = ref<any>(props.value)

function handleOnClick() {
    isEdit.value = true
    nextTick(() => {
        if (inputRef.value) {
            inputRef.value.focus()
        }
    })
}
function onChange() {
    props.onUpdateValue(inputValue.value)
    isEdit.value = false
}
function onBlur() {
    isEdit.value = false
}
</script>

<style scoped lang="scss">
.wrapper {
    cursor: pointer;
}
</style>