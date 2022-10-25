<template>
  <div class="view-wrapper">
    <n-grid
      x-gap="6"
      y-gap="6"
      cols="1 s:3"
      responsive="screen"
      style="max-width: 1280px"
    >
      <n-gi span="2">
        <n-card
          :segmented="{
            content: true,
          }"
          content-style="padding:6px"
          header-style="padding:12px"
        >
          <template #header>
            <n-h3 style="display: flex; align-items: center; margin: 0">
              <n-icon> <TextBold24Regular /> </n-icon>最新
            </n-h3>
          </template>
          <template #header-extra>
            <n-h4 style="display: flex; align-items: center; margin: 0">
              {{ `共 ${pagination.itemCount} 篇` }}
            </n-h4>
          </template>
          <n-space vertical>
            <ArticleCard
              v-if="loadingRef"
              v-for="_ in pagination.pageSize"
              :loading="true"
            ></ArticleCard>
          </n-space>
          <n-space vertical>
            <ArticleCard
              v-for="item in dataRef"
              :blog-item="item"
            ></ArticleCard>
          </n-space>
          <template #action>
            <n-pagination
              v-if="pagination.pageCount > 1"
              :page="pagination.page"
              :page-count="pagination.pageCount"
              :page-size="pagination.pageSize"
              @update-page="pagination.onChange"
              @update-page-size="pagination.onUpdatePageSize"
              style="justify-content: center"
              :show-size-picker="pagination.showSizePicker"
              :page-sizes="pagination.pageSizes"
            >
            </n-pagination>
          </template>
        </n-card>
      </n-gi>
      <n-gi span="1 s:1">
        <categories></categories>
      </n-gi>
    </n-grid>
  </div>
</template>

<script setup lang="ts">
import { TextBold24Regular } from "@vicons/fluent";
import { getBlogList } from "@/apis";
import { BlogDetailModel, BlogQuery } from "common/models";
import { useMessage } from "naive-ui";
import { AxiosError } from "axios";
import { useMemory } from "@/stores";
const loadingRef = ref(true);
const dataRef = ref([] as BlogDetailModel[]);
const message = useMessage();
const memory = useMemory();
const blogQuery = ref({
  title: undefined,
  sort_id: undefined,
  order: "desc",
  sort_by: "create_time",
  recommend: undefined,
  summary: true,
  cover: true,
} as BlogQuery);
onMounted(async () => {
  createMenuData(pagination.pageSize, pagination.page);
  memory.setTitle("首页");
});

async function createMenuData(size: number, index: number) {
  loadingRef.value = true;
  try {
    try {
      const { data, msg } = await getBlogList(size, index, blogQuery.value);
      // message.success(JSON.stringify(data))
      dataRef.value = data.list.map((item: BlogDetailModel) => {
        item.create_time = item.create_time ? new Date(item.create_time) : null;
        item.update_time = item.update_time ? new Date(item.update_time) : null;
        return item;
      });
      pagination.itemCount = data.items;
      pagination.pageCount = data.pages;
    } catch (error) {
      message.error((error as AxiosError).message);
    }
  } finally {
    loadingRef.value = false;
  }
}

const pagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: (index: number) => {
    if (!loadingRef.value) {
      createMenuData(pagination.pageSize, index).then(() => {
        pagination.page = index;
      });
    }
  },
  onUpdatePageSize: (pageSize: number) => {
    createMenuData(pageSize, pagination.page).then(() => {
      pagination.pageSize = pageSize;
    });
  },
});
</script>

<style scoped lang="scss"></style>
