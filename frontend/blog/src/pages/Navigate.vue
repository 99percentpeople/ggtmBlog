<template>
  <div class="content-wrapper">
    <page-header id="header" class="header" ref="header"></page-header>
    <router-view #="{ Component, route }" class="content">
      <transition :name="route.meta.transition">
        <component :is="Component" />
      </transition>
    </router-view>
    <page-footer ref="footer" id="footer"></page-footer>
  </div>
</template>

<style lang="scss">

.view-wrapper {
  width: inherit;
  display: flex;
  justify-content: center;
}

.navigate-wrapper {
  display: flex;
  flex-direction: column;
}
.content-wrapper {
  min-height: 100%;
  display: flex;
  flex-direction: column;
}
.content {
  padding: 6px 0;
  flex: 1 1 auto;
}
.header {
  position: sticky;
  top: 0px;
  z-index: 10;
}
</style>
