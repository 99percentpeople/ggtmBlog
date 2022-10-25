<template>
    <transition
        @before-enter="beforeEnter"
        @enter="enter"
        @after-enter="afterEnter"
        @before-leave="beforeLeave"
        @leave="leave"
        @after-leave="afterLeave"
    >
        <slot></slot>
    </transition>
</template>


<script setup lang="tsx">
const props = withDefaults(defineProps<{
    duration?: number,
    ease?: String
}>(), {
    duration: 0.3,
    ease: () => "ease-in-out"
})

const transitionStyle = `${props.duration}s height ${props.ease}`

//这些钩子是vue中transiton标签的一些钩子方法
function beforeEnter(el: any) {
    el.style.transition = transitionStyle
    if (!el.dataset)  el.dataset = {}
    el.style.height = '0'
}

function enter(el: any) {
    if (el.scrollHeight !== 0) {
        el.style.height = `${el.scrollHeight}px`
    } else {
        el.style.height = ''
    }
    el.style.overflow = 'hidden'
}

function afterEnter(el: any) {
    el.style.transition = ''
    el.style.height = ''
}

function beforeLeave(el: any) {
    if (!el.dataset) el.dataset = {}
    el.style.height = `${el.scrollHeight}px`
    el.style.overflow = 'hidden'
}

function leave(el: any) {
    if (el.scrollHeight !== 0) {
        el.style.transition = transitionStyle
        el.style.height = 0
    }
}

function afterLeave(el: any) {
    el.style.transition = ''
    el.style.height = ''
}
</script>