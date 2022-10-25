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
                    <n-icon> <ImageMultiple24Regular /> </n-icon>图片管理
                </n-h3>
            </template>
            <n-data-table
                remote
                size="small"
                ref="table"
                :bordered="false"
                :columns="columns"
                :data="dataRef"
                :loading="loadingRef"
                :pagination="pagination"
                :row-key="(row: RowData) => row.id"
            />
            <n-modal v-model:show="showModal" preset="card" style="width: 600px" :title="imgTitle">
                <img :src="previewImageUrl" style="width: 100%" />
            </n-modal>
            <template #action>
                <n-space justify="end">
                    <n-upload abstract :max="1" :custom-request="onUploadFile">
                        <n-upload-trigger #="{ handleClick }" abstract>
                            <n-button type="primary" @click="handleClick">上传文件</n-button>
                        </n-upload-trigger>
                    </n-upload>
                </n-space>
            </template>
        </n-card>
    </n-space>
</template>

<script setup lang="tsx">
import { deleteFile, getUserFiles, instance, uploadFile } from "@/apis";
import { FileQuery } from "@/models";
import { ImageMultiple24Regular } from "@vicons/fluent";
import { AxiosError, AxiosRequestConfig } from "axios";
import { FormInst, NButton, NSpace, NText, UploadCustomRequestOptions, useMessage } from "naive-ui";
const showModal = ref(false);
const previewImageUrl = ref<string | undefined>();
const imgTitle = ref<string | undefined>();
const message = useMessage();
const dataRef = ref([] as RowData[]);
const formRef = ref<FormInst | null>(null);
const loadingRef = ref(true);
const fileQuery = reactive<FileQuery>({
    name: undefined,
});

onMounted(() => {
    createFileList(pagination.pageSize, pagination.page);
});

type RowData = {
    id: number;
    file_name?: string;
    file_type?: string;
};

type Colums = {
    modify: (rowData: RowData) => void;
    delete: (rowData: RowData) => void;
    preview: (rowData: RowData) => void;
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
            key: "file_name",
            render: (rowData: RowData) => <NText>{rowData?.file_name ? rowData.file_name : "未知"}</NText>,
        },
        {
            title: "类型",
            key: "file_type",
            render: (rowData: RowData) => <NText>{rowData?.file_type ? rowData.file_type : "未知"}</NText>,
        },
        {
            title: "操作",
            key: "actions",
            width: 200,
            render: (row: RowData) => (
                <NSpace size={6}>
                    <NButton size="small" type="info" onClick={() => columns.preview(row)}>
                        预览
                    </NButton>
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
        message.info("修改" + rowData.file_name);
    },
    delete(rowData: RowData) {
        onDeleteFile(rowData.id);
    },
    preview(rowData: RowData) {
        previewImageUrl.value = `${import.meta.env.VITE_BASE_URL}/file/${rowData.id}`;
        imgTitle.value = rowData.file_name;
        showModal.value = true;
    },
});
async function onUploadFile({ file, data, headers, withCredentials, onFinish, onError, onProgress }: UploadCustomRequestOptions) {
    loadingRef.value = true;
    const formData = new FormData();
    if (data) {
        Object.keys(data).forEach((key) => {
            formData.append(key, data[key as keyof UploadCustomRequestOptions["data"]]);
        });
    }
    formData.append(file.name, file.file as File);
    try {
        const { data } = await uploadFile(formData, {
            withCredentials,
            headers,
            onUploadProgress: ({ loaded, total }) => {
                onProgress({ percent: Math.ceil((loaded / total) * 100) });
            },
        } as AxiosRequestConfig);
        file.url = `${import.meta.env.VITE_BASE_URL as string}/file/${data.id}`;
        onFinish();
        await createFileList(pagination.pageSize, pagination.page);
    } catch ({ message: msg }) {
        message.error(msg as string);
        onError();
    } finally {
        loadingRef.value = false;
    }
}
async function createFileList(size: number, index: number) {
    loadingRef.value = true;
    try {
        const { data } = await getUserFiles(size, index, fileQuery);
        dataRef.value = data.list;
        pagination.pageCount = data.pages;
        pagination.itemCount = data.items;
    } catch ({ message: msg }) {
        message.error(msg as string);
    } finally {
        loadingRef.value = false;
    }
}
async function onDeleteFile(id: number) {
    loadingRef.value = true;
    try {
        const { msg } = await deleteFile(id);
        return await createFileList(pagination.pageSize, pagination.page);
    } catch ({ message: msg }) {
        message.error(msg as string);
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
    onChange: async (index: number) => {
        if (!loadingRef.value) {
            await createFileList(pagination.pageSize, index);
            pagination.page = index;
        }
    },
    onUpdatePageSize: async (pageSize: number) => {
        await createFileList(pageSize, pagination.page);
        pagination.pageSize = pageSize;
    },
    prefix({ itemCount }: any) {
        return `共 ${itemCount} 项`;
    },
});
</script>
