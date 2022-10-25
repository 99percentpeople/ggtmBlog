<template>
  <div class="view-wrapper">
    <n-grid
      x-gap="6"
      y-gap="6"
      cols="2"
      responsive="screen"
      style="max-width: 1280px;height: min-content;"
    >
      <n-gi span="2">
        <n-card
          header-style="padding:12px"
          content-style="padding:12px"
          footer-style="padding:12px"
        >
          <template #header>
            <n-h3 style="display: flex; align-items: center; margin: 0">
              <n-icon> <List24Regular /> </n-icon>分类
            </n-h3>
          </template>
          <n-space :size="12">
            <n-button
              v-for="sort in sortItems"
              :type="blogQuery.sort_id == sort.id ? 'info' : undefined"
              @click="
                () => {
                  blogQuery.sort_id = sort.id;
                  onSearch();
                }
              "
              >{{ sort.name }}</n-button
            >
            <n-button
              v-if="blogQuery.sort_id"
              type="error"
              secondary
              round
              @click="
                () => {
                  blogQuery.sort_id = undefined;
                  onSearch();
                }
              "
            >
              <template #icon
                ><n-icon>
                  <ArrowSync20Regular />
                </n-icon>
              </template>
              重选
            </n-button>
          </n-space>
        </n-card>
      </n-gi>
      <n-gi span="2">
        <n-card
          header-style="padding:12px"
          content-style="padding:12px"
          footer-style="padding:12px"
        >
          <template #header>
            <n-h3 style="display: flex; align-items: center; margin: 0">
              <n-icon> <Tag24Regular /> </n-icon>标签
            </n-h3>
          </template>
          <n-space>
            <n-tag
              v-for="tag in tagChecked"
              checkable
              @update-checked="onUpdateCheck(tag)"
              :checked="
                tagChecked[tagChecked.findIndex((item) => item.id == tag.id)]
                  .checked
              "
              >{{ tag.name }}</n-tag
            >
          </n-space>
        </n-card>
      </n-gi>
      <n-gi span="2">
        <n-card
          :segmented="{
            content: true,
          }"
          header-style="padding:12px"
          content-style="padding:6px"
        >
          <template #header>
            <n-h3 style="display: flex; align-items: center; margin: 0">
              <n-icon> <TextBold24Regular /> </n-icon>博客
            </n-h3>
          </template>
          <template #header-extra>
            <n-h4 style="display: flex; align-items: center; margin: 0">
              {{ `共 ${BlogPagination.itemCount} 篇` }}
            </n-h4>
          </template>
          <n-empty
            description="空空如也"
            size="huge"
            v-if="blogListItems.length == 0 && !loadBlogRef"
          ></n-empty>
          <n-space vertical>
            <ArticleCard
              v-if="loadBlogRef"
              v-for="_ in BlogPagination.pageSize"
              :loading="true"
            ></ArticleCard>
          </n-space>
          <n-space vertical>
            <ArticleCard
              v-for="item in blogListItems"
              :blog-item="item"
            ></ArticleCard>
          </n-space>
          <template #action>
            <n-pagination
              v-if="BlogPagination.pageCount > 1"
              :page="BlogPagination.page"
              :page-count="BlogPagination.pageCount"
              :page-size="BlogPagination.pageSize"
              @update-page="BlogPagination.onChange"
              @update-page-size="BlogPagination.onUpdatePageSize"
              style="justify-content: center"
              :show-size-picker="BlogPagination.showSizePicker"
              :page-sizes="BlogPagination.pageSizes"
            >
            </n-pagination>
          </template>
        </n-card>
      </n-gi>
    </n-grid>
  </div>
</template>

<script setup lang="ts">
import { getBlogList, getSortList, getTagsList } from "@/apis";
import {
  BlogDetailModel,
  BlogQuery,
  Sort,
  SortQuery,
  Tag,
  TagQuery,
} from "common/models";
import { router } from "@/routers";
import {
  TextBold24Regular,
  List24Regular,
  Tag24Regular,
  ArrowSync20Regular,
} from "@vicons/fluent";
import { useMessage } from "naive-ui";
import { useMemory } from "@/stores";
const route = useRoute();
const message = useMessage();
const loadingSortRef = ref(true);
const loadingTagRef = ref(true);
const loadBlogRef = ref(true);
const sortItems = ref([] as Sort[]);
const tagItems = ref([] as Tag[]);
const blogListItems = ref([] as BlogDetailModel[]);
type TagChecked = { id: number; name: string; checked: boolean };
const tagChecked = ref([] as TagChecked[]);
function onUpdateCheck(el: TagChecked) {
  tagChecked.value[
    tagChecked.value.findIndex((item) => item.id == el.id)
  ].checked =
    !tagChecked.value[tagChecked.value.findIndex((item) => item.id == el.id)]
      .checked;

  blogQuery.value.tag_ids = tagChecked.value
    .filter((item) => item.checked)
    .map((item) => item.id)
    .join(",");
  onSearch();
}
const blogQuery = ref({
  title: route.query.title,
  sort_id: route.query.sort_id
    ? parseInt(route.query.sort_id as string)
    : undefined,
  recommend: route.query.recommend as unknown as boolean,
  order: "desc",
  sort_by: "create_time",
  summary: true,
  cover: true,
  tag_ids: route.query.tag_ids as string,
} as BlogQuery);

const sortQuery = ref({
  name: undefined,
  published: true,
} as SortQuery);
const tagQuery = ref({
  name: undefined,
  published: true,
} as TagQuery);
const memory = useMemory();
onMounted(async () => {
  memory.setTitle("分类");

  createSortData(sortPagination.pageSize, sortPagination.page);
  createTagData(tagPagination.pageSize, tagPagination.page).then(() => {
    tagChecked.value = tagItems.value.map((item) => ({
      id: item.id,
      name: item.name,
      checked:
        blogQuery.value.tag_ids
          ?.split(",")
          .findIndex((el) => parseInt(el) == item.id) != -1,
    }));
  });
  createMenuData(BlogPagination.pageSize, BlogPagination.page);
});
function onSearch() {
  router.replace({ name: "search", query: blogQuery.value as any });
  // message.info(JSON.stringify(blogQuery.value));
  createMenuData(BlogPagination.pageSize, BlogPagination.page);
}

function createTagData(size: number, index: number) {
  loadingTagRef.value = true;
  return getTagsList(size, index, tagQuery.value)
    .then(({ data, msg }) => {
      // message.success(JSON.stringify(data))
      tagItems.value = data.list;
      tagPagination.itemCount = data.items;
      tagPagination.pageCount = data.pages;
      // message.success(msg);
    })
    .catch((error) => {
      message.error(error.message);
    })
    .finally(() => {
      loadingTagRef.value = false;
    });
}

function createSortData(size: number, index: number) {
  loadingSortRef.value = true;
  return getSortList(size, index, sortQuery.value)
    .then(({ data, msg }) => {
      // message.success(JSON.stringify(data))
      sortItems.value = data.list;
      sortPagination.itemCount = data.items;
      sortPagination.pageCount = data.pages;
      // message.success(msg);
    })
    .catch((error) => {
      message.error(error.message);
    })
    .finally(() => {
      loadingSortRef.value = false;
    });
}
const sortPagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: (index: number) => {
    if (!loadingSortRef.value) {
      createSortData(sortPagination.pageSize, index).then(() => {
        sortPagination.page = index;
      });
    }
  },
  onUpdatePageSize: (pageSize: number) => {
    createSortData(pageSize, sortPagination.page).then(() => {
      sortPagination.pageSize = pageSize;
    });
  },
});
const tagPagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: (index: number) => {
    if (!loadingSortRef.value) {
      createSortData(sortPagination.pageSize, index).then(() => {
        sortPagination.page = index;
      });
    }
  },
  onUpdatePageSize: (pageSize: number) => {
    createSortData(pageSize, sortPagination.page).then(() => {
      sortPagination.pageSize = pageSize;
    });
  },
});
const BlogPagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: (index: number) => {
    if (!loadingSortRef.value) {
      createMenuData(sortPagination.pageSize, index).then(() => {
        BlogPagination.page = index;
      });
    }
  },
  onUpdatePageSize: (pageSize: number) => {
    createMenuData(pageSize, sortPagination.page).then(() => {
      BlogPagination.pageSize = pageSize;
    });
  },
});

function createMenuData(size: number, index: number) {
  loadBlogRef.value = true;
  return getBlogList(size, index, blogQuery.value)
    .then(({ data, msg }) => {
      // message.success(JSON.stringify(data))
      blogListItems.value = data.list.map((item: BlogDetailModel) => {
        item.create_time = item.create_time ? new Date(item.create_time) : null;
        item.update_time = item.update_time ? new Date(item.update_time) : null;
        return item;
      });
      BlogPagination.itemCount = data.items;
      BlogPagination.pageCount = data.pages;
      // message.success(msg);
    })
    .catch((error) => {
      message.error(error.message);
    })
    .finally(() => {
      loadBlogRef.value = false;
    });
}
</script>
