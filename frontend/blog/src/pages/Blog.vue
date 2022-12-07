<template>
  <div class="view-wrapper">
    <n-back-top style="z-index: 10" />
    <n-grid
      class="wrapper"
      x-gap="6"
      y-gap="6"
      cols="4 s:6 m:10"
      item-responsive
      responsive="screen"
    >
      <n-gi span="0 m:2" class="affix-left">
        <n-space vertical>
          <n-button
            circle
            tertiary
            size="large"
            v-if="blog.enable_comment"
            @click="commentInstRef?.focus()"
          >
            <template #icon>
              <Comment24Regular />
            </template>
          </n-button>
          <n-button circle tertiary size="large">
            <template #icon>
              <ThumbLike24Regular />
            </template>
          </n-button>
          <n-button circle tertiary size="large" @click="onToggleAnchor()">
            <template #icon>
              <Catalog />
            </template>
          </n-button>
        </n-space>
      </n-gi>
      <n-gi span="4 s:5 m:6">
        <n-card
          header-style="padding:0 20px;padding-top:6px;"
          content-style="padding:12px"
          footer-style="padding:6px 12px"
          :segmented="{
            content: true,
            footer: true,
            action: true,
          }"
        >
          <template #header>
            <n-space
              vertical
              :align="width > reactiveSize.s ? 'start' : 'center'"
            >
              <n-text tag="h3" :id="blog.title" :href="`#${blog.title}`">{{
                blog.title
              }}</n-text>
              <n-text depth="3" tag="small">
                {{ blog.summary }}
              </n-text>
            </n-space>
          </template>

          <n-space justify="space-between" align="center" style="margin: 0">
            <n-space align="center">
              <avatar size="large" :user-info="blog.user"></avatar>
              <n-tag :bordered="false" size="small">
                <template #avatar>
                  <n-icon>
                    <CalendarDay24Regular />
                  </n-icon>
                </template>
                <n-text>{{ blog.create_time?.toLocaleString() }}</n-text>
              </n-tag>
              <n-tag :bordered="false" size="small">
                <template #avatar>
                  <n-icon>
                    <Eye24Regular />
                  </n-icon>
                </template>
                <n-text>{{ blog.views }}</n-text>
              </n-tag>
            </n-space>

            <n-space align="end" :size="6">
              <n-tag type="info" size="small">{{ blog.sort?.name }}</n-tag>
              <n-tag v-for="tag in blog.tags" size="small">{{
                tag.name
              }}</n-tag>
            </n-space>
          </n-space>

          <n-space vertical align="center" style="margin: 6px 0">
            <n-image
              v-if="blog.cover"
              :src="blog.cover"
              :img-props="{
                style: `display:inline-block;max-height:250px;max-width:100%;border-radius:6px;`,
              }"
            />
          </n-space>
          <n-divider style="margin: 6px"></n-divider>

          <md-editor
            :theme="setting.isDark ? 'dark' : 'light'"
            :marked-image="markedImage"
            @get-catalog="(list) => (catalogList = renderCatalog(list))"
            previewOnly
            v-model="blog.content"
            previewTheme="github"
          ></md-editor>

          <template #footer>
            <n-space justify="center" v-if="blog.appreciation">
              <n-button round>
                赞赏
                <template #icon>
                  <n-icon>
                    <Heart24Regular />
                  </n-icon>
                </template>
              </n-button>
            </n-space>

            <n-descriptions label-placement="left" :column="1" size="small">
              <n-descriptions-item label="作者">
                <n-text depth="3">{{ blog.user.username }}</n-text>
              </n-descriptions-item>
              <n-descriptions-item label="发表时间">
                <n-text depth="3">{{
                  blog.create_time?.toLocaleString()
                }}</n-text>
              </n-descriptions-item>
              <n-descriptions-item label="最后更新" v-if="blog.update_time">
                <n-text depth="3">{{
                  blog.update_time.toLocaleString()
                }}</n-text>
              </n-descriptions-item>
              <n-descriptions-item label="版权声明" v-if="blog.share_statement">
                <n-icon color="#cc0000" size="12px"><Ban /></n-icon>
                <n-text depth="3"> 未经作者授权，禁止转载</n-text>
              </n-descriptions-item>
            </n-descriptions>
          </template>
        </n-card>
      </n-gi>
      <n-gi span="0 s:1 m:2" v-if="anchorState">
        <div class="blur-wrapper">
          <n-scrollbar class="affix-right">
            <n-anchor ignore-gap type="block" class="anchor">
              <component :is="() => catalogList" />
            </n-anchor>
          </n-scrollbar>
        </div>
      </n-gi>
      <n-gi offset="0 m:2" span="5 m:6">
        <n-card
          ref="commentRef"
          v-if="blog.enable_comment"
          header-style="padding:6px 12px"
          content-style="padding:6px;margin:0;"
          footer-style="padding:6px"
          :segmented="{
            content: 'soft',
            footer: true,
          }"
        >
          <template #header>
            <n-space align="baseline">
              评论
              <n-h4 style="display: flex; align-items: center; margin: 0">
                {{ `共 ${commentPagination.itemCount} 条` }}
              </n-h4>
            </n-space>
          </template>
          <n-space vertical>
            <visitor-comment
              v-for="c in comments"
              :model="c"
              :loading="loadingComment"
              @reply="onReply"
            ></visitor-comment>
            <n-pagination
              v-if="commentPagination.pageCount > 1"
              :page="commentPagination.page"
              :page-count="commentPagination.pageCount"
              :page-size="commentPagination.pageSize"
              @update-page="commentPagination.onChange"
              @update-page-size="commentPagination.onUpdatePageSize"
              style="justify-content: center"
              :show-size-picker="commentPagination.showSizePicker"
              :page-sizes="commentPagination.pageSizes"
            >
            </n-pagination>
          </n-space>

          <template #footer>
            <n-card
              header-style="padding:6px 12px"
              content-style="padding:6px"
              footer-style="padding:6px"
              :bordered="false"
            >
              <template #header>发布评论</template>
              <n-form label-placement="left">
                <n-grid
                  cols="1 s:2 m:4"
                  :model="comment"
                  x-gap="12"
                  responsive="screen"
                  item-responsive
                >
                  <n-form-item-gi span="4">
                    <n-input
                      ref="commentInstRef"
                      type="textarea"
                      :placeholder="
                        nameToReply ? `回复 @${nameToReply}:` : `发表你的看法`
                      "
                      :minlength="10"
                      :maxlength="200"
                      show-count
                      :autosize="{
                        minRows: 3,
                        maxRows: 5,
                      }"
                      v-model:value="comment.content"
                    ></n-input>
                  </n-form-item-gi>
                  <n-form-item-gi span="1" label="昵称">
                    <n-input
                      placeholder="请输入3~10个汉字或字母 必填"
                      v-model:value="comment.nickname"
                    ></n-input>
                  </n-form-item-gi>
                  <n-form-item-gi span="1" label="邮箱">
                    <n-input
                      placeholder="请输入邮箱 必填"
                      v-model:value="comment.email"
                    ></n-input>
                  </n-form-item-gi>
                  <n-form-item-gi span="1" label="头像">
                    <n-input
                      :input-props="{ type: 'url' }"
                      placeholder="输入头像url 可空"
                      v-model:value="comment.avatar"
                    ></n-input>
                  </n-form-item-gi>
                  <n-form-item-gi span="1" suffix>
                    <n-space
                      justify="center"
                      align="center"
                      style="width: 100%"
                      :wrap="false"
                    >
                      <n-checkbox
                        label="记住信息"
                        v-model:checked="localInfo.commentInfo.remember"
                      />
                      <n-button
                        round
                        secondary
                        type="error"
                        @click="reSetComment"
                        v-if="nameToReply"
                      >
                        <template #icon>
                          <n-icon><ArrowSync20Regular /></n-icon>
                        </template>
                        重置
                      </n-button>
                      <n-button
                        size="large"
                        type="info"
                        @click="onPublish"
                        v-if="nameToReply"
                      >
                        回复
                        <template #icon>
                          <ArrowReply24Regular />
                        </template>
                      </n-button>

                      <n-button
                        size="large"
                        type="info"
                        @click="onPublish"
                        v-else
                      >
                        发布
                        <template #icon>
                          <ArrowAutofitUp24Regular />
                        </template>
                      </n-button>
                    </n-space>
                  </n-form-item-gi>
                </n-grid>
              </n-form>
            </n-card>
          </template>
        </n-card>
      </n-gi>
    </n-grid>
  </div>
</template>

<script setup lang="tsx">
import {
  Eye24Regular,
  CalendarDay24Regular,
  Heart24Regular,
  ArrowSync20Regular,
  ArrowReply24Regular,
  ArrowAutofitUp24Regular,
  ThumbLike24Regular,
  Comment24Regular,
} from "@vicons/fluent";
import MdEditor from "md-editor-v3";

import { useSetting, useLocalInfo, useMemory } from "@/stores";
import { Ban } from "@vicons/ionicons5";
import { Catalog } from "@vicons/carbon";
import { InputInst, NAnchorLink, NCard, NImage, useMessage } from "naive-ui";
import { getBlog, getComments, postComment } from "@/apis";
import { BlogDetailModel, CommentDetail, CommentModel } from "common/models";
import { onBeforeRouteUpdate } from "vue-router";

const setting = useSetting();
const commentRef = ref<HTMLElement | null>(null);
const header = document.querySelector<HTMLElement>("#header")!;
const { height: headerHeight } = useElementSize(header);
const memory = useMemory();
const { reactiveSize } = storeToRefs(setting);
const { width } = useWindowSize();
const catalogList = ref([] as JSX.Element[]);
const route = useRoute();

const commentInstRef = ref<InputInst | null>(null);
const localInfo = useLocalInfo();
const blog = ref({
  id: parseInt(route.params["id"] as string),
  title: "",
  content: "",
  cover: "",
  flag: "",
  summary: "",
  appreciation: false,
  share_statement: "",
  enable_comment: "",
  recommend: "",
  create_time: null,
  update_time: null,
  user: {
    username: "",
  },
  tags: [],
} as unknown as BlogDetailModel);

const [anchorState, anchorToggle] = useToggle(true);
const loadingComment = ref(true);
const loadingBlog = ref(true);
const comments = ref([] as CommentDetail[]);
const comment = reactive({
  nickname: localInfo.commentInfo.remember
    ? localInfo.commentInfo.nickname
    : null,
  email: localInfo.commentInfo.remember ? localInfo.commentInfo.email : null,
  avatar: localInfo.commentInfo.avatar ? localInfo.commentInfo.avatar : null,
  content: null,
  id_ref: null,
  blog_id: parseInt(route.params["id"] as string),
} as unknown as CommentModel);

const nameToReply = ref<string | null>(null);
const message = useMessage();

onBeforeRouteUpdate(async (to, from) => {
  if (to.params.id !== from.params.id) {
    await getBlogData(parseInt(to.params.id as string));
    comment.blog_id = parseInt(to.params.id as string);
  }
  memory.setTitle(blog.value.title);
});
onMounted(async () => {
  if (blog.value.id) {
    await getBlogData(blog.value.id);
    await createComments(commentPagination.pageSize, commentPagination.page);
    memory.setTitle(blog.value.title);
    onToggleAnchor(true);
  }
});

function onToggleAnchor(value?: boolean) {
  let res = anchorToggle(value);
}

function onReply(c: CommentModel) {
  nameToReply.value = c.nickname as string;
  comment.id_ref = c.id;
  commentInstRef.value?.focus();
}
function getBlogData(id: number) {
  loadingBlog.value = true;
  return getBlog(id)
    .then(({ data, msg }) => {
      blog.value = {
        ...data,
        create_time: data.create_time ? new Date(data.create_time) : null,
        update_time: data.update_time ? new Date(data.update_time) : null,
      };
      // message.success(JSON.stringify(blog.value));
    })
    .catch((error) => {
      message.error(error.message);
    })
    .finally(() => {
      loadingBlog.value = true;
    });
}
function renderCatalog(list: any[]) {
  return list
    .map(
      (value) => new Object({ header: value, headerList: [] }) as HeadListArray
    )
    .reduce((prev, cur) => A(prev, cur), [] as HeadListArray[])
    .map((val) => B(val));
}

type HeadListArray = {
  header: any;
  headerList: HeadListArray[];
};

function A(list: HeadListArray[], val: HeadListArray) {
  let end = list.at(-1);
  if (end && val.header.level > end.header.level) {
    end.headerList = A(end.headerList, val);
  } else {
    list.push(val);
  }
  return list;
}
function B(val: HeadListArray) {
  return (
    <NAnchorLink title={val.header.text} href={`#${val.header.text}`}>
      {val.headerList.map((internalVal) => B(internalVal))}
    </NAnchorLink>
  );
}
function markedImage(href: string, _: string, desc: string) {
  return `<figure style="width:100%"><img src="${href}" alt="${desc}" style="max-height:400px"><figcaption>${desc}</figcaption></figure>`;
}
function createComments(size: number, index: number) {
  loadingComment.value = true;
  return getComments(comment.blog_id, size, index)
    .then(({ data, msg }) => {
      comments.value = data.list.map((item: CommentDetail) => {
        item.comment.create_time = item.comment.create_time
          ? new Date(item.comment.create_time)
          : null;
        item.sub_comments = item.sub_comments.map((item) => {
          item.create_time = item.create_time
            ? new Date(item.create_time)
            : null;
          return item;
        });
        return item;
      });
      commentPagination.itemCount = data.items;
      commentPagination.pageCount = data.pages;
    })
    .catch((err) => {
      message.error(err.message);
    })
    .finally(() => {
      loadingComment.value = false;
    });
}
function onPublish() {
  if (!loadingComment.value) {
    loadingComment.value = true;
    if (comment.avatar?.length === 0) {
      comment.avatar = undefined;
    }
    publishComment(comment)
      .then(() => {
        localInfo.commentInfo.nickname = localInfo.commentInfo.remember
          ? (comment.nickname as string)
          : null;
        localInfo.commentInfo.email = localInfo.commentInfo.remember
          ? (comment.email as string)
          : null;
        localInfo.commentInfo.avatar = localInfo.commentInfo.remember
          ? (comment.avatar as string)
          : null;
        reSetComment();
      })
      .finally(() => {
        loadingComment.value = false;
      });
  }
}
function reSetComment() {
  if (localInfo.commentInfo.remember == false) {
    comment.nickname = null;
    comment.email = null;
    comment.avatar = null;
  }
  (comment.content = null),
    (comment.id_ref = null),
    (comment.blog_id = parseInt(route.params["id"] as string)),
    (nameToReply.value = null);
}

function publishComment(comment: CommentModel) {
  // message.info(JSON.stringify(comment));
  return postComment(comment.blog_id, comment)
    .then((res) => {
      message.success(res.msg);
      createComments(commentPagination.pageSize, commentPagination.page);
    })
    .catch((err) => {
      message.error(err.message);
      return Promise.reject(err);
    });
}
const commentPagination = reactive({
  pageSize: 10,
  pageCount: 0,
  itemCount: 0,
  showSizePicker: true,
  pageSizes: [5, 10, 15, 20],
  page: 1,
  onChange: (index: number) => {
    if (!loadingComment.value) {
      createComments(commentPagination.pageSize, index).then(() => {
        commentPagination.page = index;
      });
    }
  },
  onUpdatePageSize: (pageSize: number) => {
    createComments(pageSize, commentPagination.page).then(() => {
      commentPagination.pageSize = pageSize;
    });
  },
});
</script>

<style scoped lang="scss">

:deep(h1),
:deep(h2),
:deep(h3),
:deep(h4),
:deep(h5),
:deep(h6) {
  &:target {
    padding-top: v-bind("`${headerHeight}px`");
  }
}

.affix-left {
  display: flex;
  justify-content: end;
  position: sticky;
  margin-top: 50%;
}
:deep(.affix-right) {
  max-height: calc(60vh - v-bind("`${headerHeight+12}px`"));
}
.blur-wrapper {
  margin: 0;
  position: sticky;
  top: v-bind("`${headerHeight+12}px`");
}
.blur-wrapper::after {
  content: "";
  position: absolute;
  top: -1px;
  left: 0;
  right: 0;
  bottom: -1px;
  backdrop-filter: blur(1000px);
  mask: linear-gradient(
    to top,
    black 0%,
    10%,
    transparent 90%,
    95%,
    black 100%
  );
  -webkit-mask: linear-gradient(
    to top,
    black 0%,
    10%,
    transparent 90%,
    95%,
    black 100%
  );
  pointer-events: none;
}
.anchor::before {
  content: "";
  display: block;
  height: 40px;
}
.anchor::after {
  content: "";
  display: block;
  height: 40px;
}

</style>

<style lang="scss">
@use "md-editor-v3/lib/style.css";
.md {
  &-dark {
    --md-bk-color: transparent;
  }
}
</style>