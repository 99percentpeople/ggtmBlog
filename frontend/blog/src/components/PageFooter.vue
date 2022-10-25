<template>
  <n-config-provider :theme-overrides="themeOverrides">
    <n-layout-footer class="footer" inverted bordered>
      <n-grid x-gap="12" cols="1 s:6 " responsive="screen">
        <n-gi span="0 s:2">
          <n-space vertical align="center">
            <n-text>最近更新</n-text>
            <div class="message">
              <n-button
                text
                tag="a"
                v-for="(blog, index) in blogMsgData"
                @click="$router.push({ name: 'blog', params: { id: blog.id } })"
              >
                <n-ellipsis style="max-width: 200px">
                  {{ blog.title }}
                </n-ellipsis>
              </n-button>
            </div>
          </n-space>
        </n-gi>
        <n-gi span="0 s:2">
          <n-space vertical align="center">
            <n-text>联系我</n-text>
            <n-space vertical align="start">
              <n-button text tag="a">Email: myzzzyearz@outlook.com</n-button>
              <n-button text tag="a">WX: yzz6787644</n-button>
            </n-space>
          </n-space>
        </n-gi>
        <n-gi span="0 s:2">
          <n-space vertical align="center" style="width: 100%">
            <n-text>简介</n-text>
            <n-text
              >个人小博客，前端采用Vue3 + Typescript + NaiveUI
              开发，后端采用Rust + Actix-web + SeaORM + PostgreSQL</n-text
            >
          </n-space>
        </n-gi>
      </n-grid>
      <n-divider style="margin: 0; padding: 0"
        >Copyright©2020-2022 ggtmBlog</n-divider
      >
      <n-space style="margin: 0" justify="center">
        <n-el tag="a" href="http://beian.miit.gov.cn">
          粤ICP备2022051746号
        </n-el>
      </n-space>
    </n-layout-footer>
  </n-config-provider>
</template>

<script setup lang="ts">
import { getBlogList } from "@/apis";
import { BlogDetailModel, BlogQuery } from "common/models";
import { GlobalThemeOverrides, darkTheme } from "naive-ui";
const blogMsgData = ref([] as BlogDetailModel[]);
onMounted(() => {
  getBlogList(9, 1, {
    order: "desc",
    sort_by: "update_time",
  } as BlogQuery).then(({ data, msg }) => {
    blogMsgData.value = data.list;
  });
});
const themeOverrides: GlobalThemeOverrides = {
  common: {
    textColor1: darkTheme.common?.textColor1,
    textColor2: darkTheme.common?.textColor2,
    textColor3: darkTheme.common?.textColor3,
    primaryColor: darkTheme.common?.primaryColor,
    primaryColorHover: darkTheme.common?.primaryColorHover,
  },
  Divider: {
    color: darkTheme.Divider.common?.borderColor,
  },
};
</script>

<style scoped lang="scss">
@use "@/styles/reactive.module";

.footer {
  padding: 22px;
}
.message {
  display: grid;
  grid-template-rows: repeat(3, 1fr);
  grid-gap: 12px 12px;
  grid-auto-flow: row;
  justify-items: start;
}

@media only screen and (min-width: reactive.$l) {
  .message {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media only screen and (max-width: reactive.$m) {
  .message {
    grid-template-columns: repeat(1, 1fr);
  }
}
</style>
