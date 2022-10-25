<template>
    <n-el>
        <div class="y-handle" @mousedown="mouseDown">
            <div class="bar"></div>
        </div>
    </n-el>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    (event: "heightChange", height: number): void;
}>();
const lastY = ref(0);

onBeforeMount(() => {
    document.addEventListener("mouseup", mouseUp);
});
onBeforeUnmount(() => {
    document.removeEventListener("mouseup", mouseUp);
});

function mouseDown(event: MouseEvent) {
    document.addEventListener("mousemove", mouseMove);
    lastY.value = event.screenY;
}
function mouseMove(event: MouseEvent) {
    emit("heightChange", lastY.value - event.screenY);
    lastY.value = event.screenY;
    event.preventDefault();
}
function mouseUp(event: MouseEvent) {
    lastY.value = 0;
    document.removeEventListener("mousemove", mouseMove);
}
</script>
<style scoped lang="scss">
.y-handle {
    height: 0;
    width: inherit;
    z-index: 10;
}
.bar {
    position: relative;
    width: inherit;
    height: 2px;
    background: var(--border-color);
    transition: all 0.1s ease;
    cursor: n-resize;

    &:hover {
        transform: translateY(-50%);
        height: 12px;
    }
}
</style>
