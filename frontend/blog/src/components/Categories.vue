<template>
  <div class="section">
    <div class="space">
      <n-card size="small" title="分类">
        <template #header-extra>
          <router-link :to="{ name: 'search' }">
            <n-button text tag="a" icon-placement="right">
              更多
              <template #icon>
                <n-icon>
                  <ChevronDoubleRight20Regular />
                </n-icon>
              </template>
            </n-button>
          </router-link>
        </template>
        <n-space>
          <template v-for="sort in sortItems">
            <n-badge v-if="sort.blog_count" :value="sort.blog_count">
              <n-button
                @click="
                  $router.push({ name: 'search', query: { sort_id: sort.id } })
                "
                >{{ sort.name }}</n-button
              >
            </n-badge>
            <n-button
              v-else
              @click="
                $router.push({ name: 'search', query: { sort_id: sort.id } })
              "
              >{{ sort.name }}</n-button
            >
          </template>
        </n-space>
      </n-card>
    </div>
    <div class="space">
      <n-card size="small" title="标签">
        <template #header-extra>
          <router-link :to="{ name: 'search' }">
            <n-button text tag="a" icon-placement="right">
              更多
              <template #icon>
                <n-icon>
                  <ChevronDoubleRight20Regular />
                </n-icon>
              </template>
            </n-button>
          </router-link>
        </template>
        <n-space>
          <template v-for="tag in tagItems">
            <n-badge v-if="tag.blog_count" :value="tag.blog_count">
              <n-button
                @click="
                  $router.push({ name: 'search', query: { tag_ids: tag.id } })
                "
                >{{ tag.name }}</n-button
              >
            </n-badge>
            <n-button
              v-else
              @click="
                $router.push({ name: 'search', query: { tag_ids: tag.id } })
              "
              >{{ tag.name }}</n-button
            >
          </template>
        </n-space>
      </n-card>
    </div>
    <div class="sticky-aside">
      <n-card size="small" title="最新推荐">
        <n-list>
          <categories-card
            v-for="_ in BlogPagination.pageSize"
            v-if="loadBlogRef"
            :loading="loadBlogRef"
          ></categories-card>
          <categories-card
            v-for="blog in blogListItems"
            :item="blog"
            v-else
          ></categories-card>
        </n-list>
        <template #action>
          <contact-link />
        </template>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="tsx">
import { getBlogList, getSortList, getTagsList } from "@/apis";
import {
  BlogDetailModel,
  BlogQuery,
  Sort,
  SortQuery,
  Tag,
  TagQuery,
} from "common/models";
import { ChevronDoubleRight20Regular } from "@vicons/fluent";
import { NBadge, useMessage } from "naive-ui";
import { AxiosError } from "axios";
const header = ref<HTMLElement | null>(document.getElementById("header"));
const { height: headerHeight } = useElementSize(header);
const sortItems = ref([] as Sort[]);
const tagItems = ref([] as Tag[]);
const blogListItems = ref([] as BlogDetailModel[]);
const loadingSortRef = ref(true);
const loadingTagRef = ref(true);
const loadBlogRef = ref(true);
const message = useMessage();

const sortQuery = ref({
  name: undefined,
  published: true,
} as SortQuery);
const tagQuery = ref({
  name: undefined,
  published: true,
} as TagQuery);

const blogQuery = ref({
  title: undefined,
  sort_id: undefined,
  recommend: true,
  order: "desc",
  sort_by: "create_time",
  summary: false,
  cover: false,
  tag_ids: undefined,
} as BlogQuery);
onMounted(async () => {
  createSortData(sortPagination.pageSize, sortPagination.page);
  createTagData(tagPagination.pageSize, tagPagination.page);
  createBlogData(BlogPagination.pageSize, BlogPagination.page);
});
async function createTagData(size: number, index: number) {
  loadingTagRef.value = true;
  try {
    const { data, msg } = await getTagsList(size, index, tagQuery.value);
    // message.success(JSON.stringify(data))
    tagItems.value = data.list;
    tagPagination.itemCount = data.items;
    tagPagination.pageCount = data.pages;
  } catch (error) {
    message.error((error as AxiosError).message);
  } finally {
    loadingTagRef.value = false;
  }
}

async function createSortData(size: number, index: number) {
  loadingSortRef.value = true;
  try {
    const { data, msg } = await getSortList(size, index, sortQuery.value);
    // message.success(JSON.stringify(data))
    sortItems.value = data.list;
    sortPagination.itemCount = data.items;
    sortPagination.pageCount = data.pages;
  } catch (error) {
    message.error((error as AxiosError).message);
  } finally {
    loadingSortRef.value = false;
  }
}
const sortPagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: async (index: number) => {
    if (!loadingSortRef.value) {
      await createSortData(sortPagination.pageSize, index);
      sortPagination.page = index;
    }
  },
  onUpdatePageSize: async (pageSize: number) => {
    await createSortData(pageSize, sortPagination.page);
    sortPagination.pageSize = pageSize;
  },
});
const tagPagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: async (index: number) => {
    if (!loadingSortRef.value) {
      await createTagData(tagPagination.pageSize, index);
      tagPagination.page = index;
    }
  },
  onUpdatePageSize: async (pageSize: number) => {
    await createTagData(pageSize, tagPagination.page);
    tagPagination.pageSize = pageSize;
  },
});
const BlogPagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: async (index: number) => {
    if (!loadingSortRef.value) {
      await createBlogData(BlogPagination.pageSize, index);
      BlogPagination.page = index;
    }
  },
  onUpdatePageSize: async (pageSize: number) => {
    await createBlogData(pageSize, BlogPagination.page);
    BlogPagination.pageSize = pageSize;
  },
});

async function createBlogData(size: number, index: number) {
  loadBlogRef.value = true;
  try {
    const { data, msg } = await getBlogList(size, index, blogQuery.value);
    blogListItems.value = data.list.map((item: BlogDetailModel) => {
      item.create_time = item.create_time ? new Date(item.create_time) : null;
      item.update_time = item.update_time ? new Date(item.update_time) : null;
      return item;
    });
    BlogPagination.itemCount = data.items;
    BlogPagination.pageCount = data.pages;
  } catch (error) {
    message.error((error as AxiosError).message);
  } finally {
    loadBlogRef.value = false;
  }
}
</script>

<style scoped lang="scss">
.sticky-aside {
  position: sticky;
  top: v-bind("`${headerHeight + 6}px`");
  max-height: calc(100vh - v-bind("`${headerHeight}px`"));
  overflow: auto;
}
.section {
  display: flex;
  flex-direction: column;
  height: 100%;
}
.space {
  padding-bottom: 6px;
}
</style>
