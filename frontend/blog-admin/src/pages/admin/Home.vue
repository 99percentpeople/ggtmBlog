<template>
    <n-space vertical align="stretch" item-style="display:flex;flex-direction:column;align-items:center">
        <n-card
            style="max-width: 1280px"
            :segmented="{
                content: true,
            }"
            header-style="padding:12px"
        >
            <template #header>
                <n-h3 style="display: flex; align-items: center; margin: 0">
                    <n-icon> <List24Regular /> </n-icon>管理
                </n-h3>
            </template>
            <n-form ref="formRef" inline :model="blogQuery" size="medium" label-placement="left">
                <n-form-item path="title">
                    <n-input v-model:value="blogQuery.title" clearable placeholder="标题" />
                </n-form-item>
                <n-form-item style="min-width: 100px" path="type">
                    <sort-select v-model="blogQuery.sort_id"></sort-select>
                </n-form-item>
                <n-form-item label="推荐" path="recommend">
                    <n-checkbox v-model:checked="blogQuery.recommend" />
                </n-form-item>
                <n-form-item>
                    <n-button attr-type="button" @click="onSearch">搜索</n-button>
                </n-form-item>
            </n-form>
            <n-data-table size="small" remote :loading="loadingRef" :bordered="false" :columns="columns" :data="dataRef" :pagination="pagination" />
            <template #action>
                <n-space justify="end">
                    <n-button type="primary" @click="onNewBlog">新增</n-button>
                </n-space>
            </template>
        </n-card>
    </n-space>
</template>

<script setup lang="tsx">
import { deleteBlog, getBlogList } from "@/apis";
import { BlogDetail, BlogQuery } from "@/models";
import { List24Regular } from "@vicons/fluent";
import { FormInst, NButton, NCheckbox, NEllipsis, NSpace, NTag, NText, useMessage } from "naive-ui";
import { TableColumn } from "naive-ui/lib/data-table/src/interface";
const formRef = ref<FormInst | null>(null);
const message = useMessage();
const router = useRouter();
onMounted(() => {
    createMenuData(pagination.pageSize, pagination.page);
});

const blogQuery = reactive({
    title: null,
    sort_id: null,
    order: null,
    sort_by: null,
    recommend: undefined,
    tag_ids: null,
} as BlogQuery);

const dataRef = ref([] as BlogDetail[]);
const loadingRef = ref(true);
function createMenuData(size: number, index: number) {
    loadingRef.value = true;
    return getBlogList(size, index, blogQuery)
        .then(({ data, msg }) => {
            dataRef.value = data.list.map((item: BlogDetail) => {
                item.create_time = item.create_time ? new Date(item.create_time) : null;
                item.update_time = item.update_time ? new Date(item.update_time) : null;
                return item;
            });
            pagination.itemCount = data.items;
            pagination.pageCount = data.pages;
        })
        .catch((error) => {
            message.error(error.message);
        })
        .finally(() => {
            loadingRef.value = false;
        });
}

function onSearch(e: MouseEvent) {
    formRef.value?.validate((errors) => {
        if (!errors) {
            createMenuData(pagination.pageSize, pagination.page);
        } else {
            message.info(errors.toString());
        }
    });
}

function onNewBlog(e: MouseEvent) {
    router.push({ name: "publish" });
}

type Colums = {
    modify: (rowData: BlogDetail) => void;
    delete: (rowData: BlogDetail) => void;
};

const createColumns = (columns: Colums) => {
    return [
        {
            type: "expand",
            expandable: (rowData: BlogDetail) => rowData.summary,
            renderExpand: (rowData: BlogDetail) => <NEllipsis>{rowData.summary}</NEllipsis>,
        },
        {
            title: "id",
            key: "id",
            width: 50,
        },
        {
            title: "标题",
            key: "title",
            ellipsis: {
                tooltip: true,
            },
        },
        {
            title: "作者",
            key: "author",
            render: (rowData: BlogDetail) => <NText>{rowData.user.username}</NText>,
        },
        {
            title: "状态",
            key: "published",
            render: (rowData: BlogDetail) => <NText>{rowData.published ? "已发布" : "未发布"}</NText>,
        },
        {
            title: "推荐",
            key: "recommend",
            render: (rowData: BlogDetail) => <NText>{rowData.recommend ? "是" : "否"}</NText>,
        },
        {
            title: "更新时间",
            key: "update_time",
            render: (rowData: BlogDetail) => <NText>{rowData.update_time ? rowData.update_time.toLocaleString() : "未更新"}</NText>,
        },
        {
            title: "分类",
            key: "sort",
            render: (rowData: BlogDetail) => <> {rowData.sort ? <NTag type="info">{rowData.sort?.name}</NTag> : <NText depth={3}>无</NText>}</>,
        },
        {
            title: "标签",
            key: "tags",
            render: (row: BlogDetail) => (
                <NSpace size={6}>
                    {row.tags.map((tag) => (
                        <NTag>{tag.name}</NTag>
                    ))}
                </NSpace>
            ),
            width: 280,
        },
        {
            title: "操作",
            key: "actions",
            render: (row: BlogDetail) => (
                <NSpace size={6}>
                    <NButton size="small" type="info" onClick={() => columns.modify(row)}>
                        修改
                    </NButton>
                    <NButton size="small" type="error" onClick={() => columns.delete(row)}>
                        删除
                    </NButton>
                </NSpace>
            ),
            width: 130,
        },
    ] as TableColumn[];
};

const columns = createColumns({
    modify(rowData: BlogDetail) {
        router.push({ name: "publish", params: { blog_id: rowData.id } });
    },
    delete(rowData: BlogDetail) {
        deleteBlog(rowData.id)
            .then((data) => {
                message.success(data.msg);
                dataRef.value.splice(
                    dataRef.value.findIndex((data) => data.id == rowData.id),
                    1
                );
                return createMenuData(pagination.pageSize, pagination.page).then(() => {
                    if (dataRef.value.length == 0) {
                    }
                });
            })
            .catch((error) => {
                message.error(error.response.data);
            });
    },
});
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
    prefix({ itemCount }: any) {
        return `共 ${itemCount} 项`;
    },
});
</script>

<style scoped></style>
