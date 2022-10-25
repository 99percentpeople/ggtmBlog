<template>
  <n-config-provider
    :theme-overrides="isDark ? darkThemeOverrides : lightThemeOverrides"
  >
    <n-el>
      <n-layout-header
        class="app-header"
        inverted
        bordered
        style="display: flex; justify-content: center; padding: 6px 0"
      >
        <n-grid
          :x-gap="6"
          cols="2 m:6"
          responsive="screen"
          style="width: 100%; max-width: 1280px"
        >
          <n-gi span="2 m:3">
            <div class="title" :wrap="false">
              <router-link :to="{ name: 'home' }">
                <h2>
                  <a>ggtmBlog</a>
                </h2>
              </router-link>
              <n-auto-complete
                :placeholder="searchPlaceholder"
                :loading="inputLoading"
                :options="options"
                v-model:value="title_or_content"
                :render-label="renderLabel"
                :show="showDropdown"
                @update:value="onSearch"
                @select="handleSelect"
                clearable
                clear-after-select
                blur-after-select
              >
              </n-auto-complete>

              <n-button
                #icon
                @click="menuToggle()"
                v-if="reactiveSize.s > width"
                ><AppsList20Regular
              /></n-button>
            </div>
          </n-gi>

          <n-gi span="2 m:3">
            <collapse-transition :duration="0.3" ease="ease">
              <div
                v-if="menuState || width > reactiveSize.s"
                :style="{
                  display: width > reactiveSize.s ? 'flex' : 'block',
                  justifyContent:
                    width > reactiveSize.m ? 'flex-end' : 'space-around',
                  alignItems: 'center',
                }"
              >
                <div style="display: flex; align-items: center">
                  <n-switch
                    @click="setting.toggleDark()"
                    :value="isDark"
                    size="large"
                    v-if="reactiveSize.s < width"
                  >
                    <template #checked>
                      <n-icon><WeatherMoon24Regular /></n-icon>
                    </template>
                    <template #unchecked>
                      <n-icon><WeatherSunny24Filled /></n-icon>
                    </template>
                  </n-switch>
                </div>
                <n-menu
                  :mode="width > reactiveSize.s ? 'horizontal' : 'vertical'"
                  :root-indent="12"
                  :indent="6"
                  :options="menuOptions"
                  inverted
                  :value="$route.name?.toString()"
                ></n-menu>
              </div>
            </collapse-transition>
          </n-gi>
        </n-grid>
      </n-layout-header>
    </n-el>
  </n-config-provider>
</template>

<script setup lang="tsx">
import {
  Home16Regular,
  Shortpick20Regular,
  Info16Regular,
  AppsList20Regular,
  AppsListDetail24Regular,
  Guest24Regular,
  WeatherSunny24Filled,
  WeatherMoon24Regular,
} from "@vicons/fluent";
import {
  GlobalThemeOverrides,
  AutoCompleteOption,
  darkTheme,
  NEllipsis,
  NSpace,
  NTag,
  NEl,
} from "naive-ui";
import { useToggle, useWindowSize } from "@vueuse/core";
import { useSetting } from "@/stores";
import { RouterLink } from "vue-router";
import { VNodeChild } from "vue";
import { getBlogSearchResult } from "@/apis";

const setting = useSetting();
const showDropdown = ref(false);

const { reactiveSize, isDark } = storeToRefs(setting);
const { width } = useWindowSize();
const inputLoading = ref(false);
const options = ref([] as AutoCompleteOption[]);
const router = useRouter();
const title_or_content = ref<string>("");
const searchPlaceholder = "搜索";
function renderLabel(option: AutoCompleteOption): VNodeChild {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-around",
        alignItems: "center",
        flexWrap: "nowrap",
      }}
    >
      <NTag type="info">{"文章"}</NTag>
      <NEllipsis>{option.label}</NEllipsis>
    </div>
  );
}
function handleSelect(id: number | string) {
  router.push({
    name: "blog",
    params: { id },
  });

  showDropdown.value = false;
  options.value = [];
  title_or_content.value = "";
}
function onSearch(val: string) {
  if (val) {
    showDropdown.value = true;
    title_or_content.value = val;
    inputLoading.value = true;
    getBlogSearchResult({ title_or_content: title_or_content.value })
      .then((res) => {
        options.value = res.map((item) => {
          return {
            label: item.title,
            value: item.id.toString(),
          };
        });
      })
      .finally(() => {
        inputLoading.value = false;
      });
  } else {
    showDropdown.value = false;
    options.value = [];
  }
}
const [menuState, menuToggle] = useToggle(false);
const menuOptions = [
  {
    label: () => <RouterLink to={{ name: "home" }}>首页</RouterLink>,
    key: "home",
    icon: () => <Home16Regular />,
  },
  {
    label: () => <RouterLink to={{ name: "search" }}>分类</RouterLink>,
    key: "search",
    icon: () => <AppsListDetail24Regular />,
  },
  {
    label: () => <RouterLink to={{ name: "archive" }}>归档</RouterLink>,
    key: "archive",
    icon: () => <Shortpick20Regular />,
  },
  {
    label: () => <RouterLink to={{ name: "about" }}>关于</RouterLink>,
    key: "about",
    icon: () => <Info16Regular />,
  },
  {
    label: () => (
      <a href={location.origin + import.meta.env.VITE_ADMIN_URL}>后台</a>
    ),
    key: "admin",
    icon: () => <Guest24Regular />,
  },
];

const lightThemeOverrides: GlobalThemeOverrides = {
  common: {
    textColor1: darkTheme.common?.textColor1,
    // textColor2: darkTheme.common?.textColor2,
    textColor3: darkTheme.common?.textColor3,
    primaryColor: darkTheme.common?.primaryColor,
    primaryColorHover: darkTheme.common?.primaryColorHover,
  },
  Input: {
    textColor: darkTheme.common?.textColor2,
    color: darkTheme.Input.common?.bodyColor,
    colorFocus: darkTheme.Input.common?.inputColor,
  },
  Button: {
    textColor: darkTheme.Button.common?.textColor2,
  },
  Switch: {
    railColor: darkTheme.Switch.common?.railColor,
  },
  // ...
};
const darkThemeOverrides: GlobalThemeOverrides = {
  // ...
};
</script>
<style scoped lang="scss">
.title {
  display: flex;
  & > * {
    margin: 3px;
  }
  position: relative;
  width: inherit;
}
</style>
