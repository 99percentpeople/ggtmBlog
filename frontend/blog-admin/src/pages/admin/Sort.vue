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
                    <n-icon> <List24Regular /> </n-icon>分类管理
                </n-h3>
            </template>
            <n-form ref="formRef" inline :model="formValue" size="medium" label-placement="left">
                <n-form-item path="name" label="名称" style="width: 100%">
                    <n-input v-model:value="formValue.name" />
                </n-form-item>
                <n-form-item>
                    <n-button type="info" @click="onAddTypes">添加</n-button>
                </n-form-item>
            </n-form>
            <n-data-table
                remote
                ref="table"
                size="small"
                :bordered="false"
                :columns="columns"
                :data="dataRef"
                :loading="loadingRef"
                :pagination="pagination"
                :row-key="(row: RowData) => row.id"
            />
        </n-card>
    </n-space>
</template>

<script setup lang="tsx">
import axios from "axios";
import { List24Regular } from "@vicons/fluent";
import { FormInst, NButton, NSpace, NTag, NText, useMessage, DataTableInst, PaginationProps } from "naive-ui";
import ShowEdit from "@/components/ShowEdit.vue";
import { deleteSort, getSortList, postOneSort, putSort } from "@/apis";
import { SortQuery, TagQuery } from "@/models";
const message = useMessage();
let formValue = ref({
    name: "",
});

const dataRef = ref([] as RowData[]);
const formRef = ref<FormInst | null>(null);
const loadingRef = ref(true);
const sortQuery = reactive<SortQuery>({
    name: undefined,
    published: undefined,
});
function createSortList(size: number, index: number) {
    loadingRef.value = true;
    return getSortList(size, index, sortQuery)
        .then(({ data }) => {
            dataRef.value = data.list;
            pagination.itemCount = data.items;
            pagination.pageCount = data.pages;
        })
        .finally(() => {
            loadingRef.value = false;
        });
}

onMounted(() => {
    createSortList(pagination.pageSize, pagination.page);
});
function onAddTypes(e: MouseEvent) {
    e.preventDefault();
    formRef.value?.validate((errors) => {
        if (!errors) {
            postOneSort(formValue.value.name)
                .then((data) => {
                    message.success(data.msg);

                    dataRef.value.push(data.data);
                    formValue.value.name = "";
                    return createSortList(pagination.pageSize, pagination.page);
                })
                .catch((error) => {
                    message.error(error.response.data);
                });
        } else {
            message.error(errors.toString());
        }
    });
}

type RowData = {
    id: number;
    name: string;
};
type Colums = {
    modify: (rowData: RowData) => void;
    delete: (rowData: RowData) => void;
};

const createColumns = (columns: Colums) => {
    return [
        {
            title: "id",
            key: "id",
            width: 100,
        },
        {
            title: "名称",
            key: "name",
            render: (row: RowData) => (
                <ShowEdit
                    value={row.name}
                    onUpdateValue={(val) => {
                        putSort(row.id, val)
                            .then((data) => {
                                message.success(data.msg);
                                return createSortList(pagination.pageSize, pagination.page);
                            })
                            .catch((err) => {
                                message.error(err.response.data);
                            });
                    }}
                ></ShowEdit>
            ),
        },
        {
            title: "相关博客数",
            key: "blog_count",
        },
        {
            title: "操作",
            key: "actions",
            width: 200,
            render: (row: RowData) => (
                <NSpace size={6}>
                    <NButton size="small" type="error" onClick={() => columns.delete(row)}>
                        删除
                    </NButton>
                </NSpace>
            ),
        },
    ];
};

const columns = createColumns({
    modify(rowData: RowData) {
        message.info("修改" + rowData.name);
    },
    delete(rowData: RowData) {
        deleteSort(rowData.id)
            .then((data) => {
                message.success(data.msg);
                dataRef.value.splice(
                    dataRef.value.findIndex((data) => data.id == rowData.id),
                    1
                );
                return createSortList(pagination.pageSize, pagination.page).then(() => {
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
            createSortList(pagination.pageSize, index).then(() => {
                pagination.page = index;
            });
        }
    },
    onUpdatePageSize: (pageSize: number) => {
        createSortList(pageSize, pagination.page).then(() => {
            pagination.pageSize = pageSize;
        });
    },
    prefix({ itemCount }: any) {
        return `共 ${itemCount} 项`;
    },
});
</script>
