<template>
    <navigate>
        <router-view #="{ Component, route }">
            <transition :name="route.meta.transition">
                <component :is="Component" />
            </transition>
        </router-view>
    </navigate>
</template>

<style scoped lang="scss">

</style>