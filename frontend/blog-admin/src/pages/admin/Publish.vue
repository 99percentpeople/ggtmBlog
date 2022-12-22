<template>
    <n-space vertical align="stretch" item-style="display:flex;flex-direction:column;align-items:center">
        <n-card style="max-width: 1440px" :segmented="{
            content: true,
        }" header-style="padding:12px">
            <template #header>
                <n-h3 style="display: flex; align-items: center; margin: 0">
                    <n-icon>
                        <AddSquareMultiple20Regular />
                    </n-icon>发布
                </n-h3>
            </template>
            <n-form ref="formRef" :model="formValue" size="medium" label-placement="top" :rules="rules">
                <n-grid cols="2 s:5 m:6" x-gap="12" item-responsive responsive="screen">
                    <n-form-item-gi label="文章类型" path="flag">
                        <n-select v-model:value="formValue.flag" placeholder="类型" :options="selectOptions" />
                    </n-form-item-gi>
                    <n-form-item-gi label="分类" path="sort">
                        <sort-select v-model="formValue.sort_id"></sort-select>
                    </n-form-item-gi>
                    <n-form-item-gi span="1 m:2" label="标签" path="tag_ids">
                        <tag-select v-model="formValue.tag_ids"></tag-select>
                    </n-form-item-gi>
                    <n-form-item-gi span="2">
                        <n-checkbox label="发布" v-model:checked="formValue.published" />
                        <n-checkbox label="推荐" v-model:checked="formValue.recommend" />
                        <n-checkbox label="赞赏" v-model:checked="formValue.appreciation" />
                        <n-checkbox label="评论" v-model:checked="formValue.enable_comment" />
                        <n-checkbox label="转载声明" v-model:checked="formValue.share_statement" />
                    </n-form-item-gi>

                    <n-form-item-gi span="10" label="标题" path="title" style="width: 100%">
                        <n-input v-model:value="formValue.title" placeholder="标题" />
                    </n-form-item-gi>
                    <n-form-item-gi span="10" label="描述" path="summary" style="width: 100%">
                        <n-input type="textarea" :autosize="{
                            minRows: 3,
                            maxRows: 5,
                        }" v-model:value="formValue.summary" placeholder="描述" />
                    </n-form-item-gi>
                    <n-form-item-gi span="10" path="content" style="width: 100%" label="正文">
                        <div style="display: flex; flex-direction: column; width: 100%">
                            <md-editor @upload-img="onUploadEditorImg" @save="onSave" :toolbars-exclude="['github']"
                                v-model="formValue.content" :style="{ height: `${mdEditorHeight}px` }" />
                            <y-handle @height-change="handleHeightChange"></y-handle>
                        </div>
                    </n-form-item-gi>
                    <n-form-item-gi span="10" label="封面图片">
                        <n-upload v-model:file-list="fileList" list-type="image-card" :max="1"
                            :custom-request="onUploadFile" @preview="handlePreview" @change="handleUploadChange"
                            @update:file-list="handleFileListChange" @before-upload="beforeUpload"
                            @remove="handleRemoveFile" />
                        <n-modal v-model:show="showModal" preset="card" style="width: 600px">
                            <img :src="previewImageUrl" style="width: 100%" />
                        </n-modal>
                    </n-form-item-gi>
                </n-grid>
            </n-form>
            <template #action>
                <n-space justify="end">
                    <n-button secondary @click="$router.back()">返回</n-button>
                    <n-button type="info" @click="onSave">{{ isNew ? "发布" : "保存" }}</n-button>
                </n-space>
            </template>
        </n-card>
    </n-space>
</template>

<script setup lang="tsx">
import { AddSquareMultiple20Regular } from "@vicons/fluent";
import { FormInst, FormItemRule, FormRules, UploadCustomRequestOptions, UploadFileInfo, useMessage } from "naive-ui";
import { BlogModel, Tag } from "@/models";
import MdEditor from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import { useLoginStatus } from "@/stores";
import { getBlog, postOneBlog, putBlog, instance, uploadFile, deleteFile } from "@/apis";
import { AxiosError, AxiosRequestConfig, AxiosResponse } from "axios";
const route = useRoute();
const formRef = ref<FormInst | null>(null);
const mdEditorHeight = ref(500);
const message = useMessage();
const loginStatus = useLoginStatus();
const router = useRouter();
const isNew = ref(true);
const removeImg = ref<number | undefined>(undefined);
const loadingBlogRef = ref<boolean | undefined>(true);
let blogId = parseInt(route.params["blog_id"] as string);
onMounted(() => {
    if (!isNaN(blogId)) {
        createBlogData(blogId);
        isNew.value = false;
    }
});

const formValue = ref({
    id: undefined,
    title: "",
    content: "",
    cover: "",
    flag: null,
    appreciation: false,
    share_statement: false,
    enable_comment: false,
    recommend: false,
    sort: null,
    views: 0,
    summary: null,
    user: null,
    tags: [] as Tag[],
    user_id: loginStatus.userInfo.id,
    tag_ids: [] as number[],
    sort_id: null,
    published: false,
});

async function createBlogData(id: number) {
    loadingBlogRef.value = true;
    try {
        const { data, msg } = await getBlog(id);
        formValue.value = {
            tag_ids: data.tags.map((tag: Tag) => tag.id),
            sort_id: data.sort.id,
            user_id: data.user.id,
            ...data,
        };
        if (data.cover) {
            fileList.value.push({
                id: data.cover.split("/").at(-1),
                name: data.cover,
                status: "finished",
                url: data.cover,
            });
        }
        message.success(msg);
    } catch (error) {
        message.error((error as AxiosError).message);
        if (((error as AxiosError).response as AxiosResponse).status === 404) {
            router.push({ name: "publish" });
        }
    } finally {
        loadingBlogRef.value = false;
    }
}

async function onBlogPublish() {
    formRef.value?.validate(async (errors) => {
        if (!errors) {
            try {
                const { data, msg } = await postOneBlog(formValue.value as unknown as BlogModel);
                formValue.value.id = data.id;
                message.success(msg);
            } catch ({ message: msg, response }) {
                message.error(msg as string);
            }
        }
    });
}

async function onBlogUpdate() {
    formRef.value?.validate(async (errors) => {
        if (!errors) {
            loadingBlogRef.value = true;
            try {
                const { msg } = await putBlog(blogId, formValue.value as unknown as BlogModel);
                message.success(msg);
            } catch ({ message: msg }) {
                message.error(msg as string);
            } finally {
                loadingBlogRef.value = false;
            }
        } else {
            message.info(JSON.stringify(errors));
        }
    });
}

function handleHeightChange(h: number): void {
    mdEditorHeight.value -= h;
    if (mdEditorHeight.value >= 1000) {
        mdEditorHeight.value = 1000;
    } else if (mdEditorHeight.value <= 300) {
        mdEditorHeight.value = 300;
    }
}
async function onSave() {
    if (isNew.value) {
        await onBlogPublish();
        isNew.value = false;
        blogId = formValue.value.id as unknown as number;
        router.replace({ name: "publish", params: { blog_id: formValue.value.id } });
    } else {
        await onBlogUpdate();
    }
    if (removeImg.value) {
        await onRemoveFile(removeImg.value);
        removeImg.value = undefined;
    }
}
async function onRemoveFile(id: number) {
    try {
        await deleteFile(id);
    } catch ({ message: msg }) {
        message.error(msg as string);
    }
}

async function beforeUpload(data: { file: UploadFileInfo; fileList: UploadFileInfo[] }) {
    if (data.file.file?.type !== "image/png" && data.file.file?.type !== "image/jpeg") {
        message.error("只能上传png或jpg格式的图片文件，请重新选择");
        return false;
    }
    return true;
}
const selectOptions = [
    {
        label: "原创",
        value: "Original",
    },
    {
        label: "转载",
        value: "Reprint",
    },
    {
        label: "翻译",
        value: "Translate",
    },
];

const rules: FormRules = {
    title: [
        {
            required: true,
            trigger: ["blur", "input"],
            validator(rule: FormItemRule, value: string) {
                if (!value) {
                    throw Error("请输入文章标题");
                } else if (value.length < 5) {
                    throw Error("标题长度必须大于等于5个字符");
                }
                return true;
            },
        },
    ],
    content: {
        required: true,
        trigger: ["blur", "change"],
        validator(rule: FormItemRule, value: string) {
            if (!value) {
                throw Error("请输入文章正文");
            } else if (value.length < 20) {
                throw Error("正文长度必须大于等于20个字符");
            }
            return true;
        },
    },
    flag: {
        required: true,
        trigger: ["blur", "change"],
        validator(rule: FormItemRule, value: string) {
            console.log(value);
            if (!value) {
                throw Error("需要选择文章类型");
            }
            return true;
        },
    },
};

const showModal = ref(false);
const previewImageUrl = ref("");
const fileList = ref<UploadFileInfo[]>([]);
function handlePreview(file: UploadFileInfo) {
    const { url } = file;
    message.info(url as string);
    previewImageUrl.value = url as string;
    showModal.value = true;
}

async function onUploadFile({ file, data, headers, withCredentials, onFinish, onError, onProgress }: UploadCustomRequestOptions) {
    const formData = new FormData();
    if (data) {
        Object.keys(data).forEach((key) => {
            formData.append(key, data[key as keyof UploadCustomRequestOptions["data"]]);
        });
    }
    formData.append(file.name, file.file as File);
    try {
        const { data, msg } = await uploadFile(formData, {
            withCredentials,
            headers,
            onUploadProgress: ({ loaded, total }) => {
                onProgress({ percent: Math.ceil((loaded / total) * 100) });
            },
        } as AxiosRequestConfig);
        // console.log(data);
        file.url = `${import.meta.env.VITE_API_URL}/file/${data[0].id}`;
        onFinish();
    } catch (error) {
        message.error((error as AxiosError).message);
        onError();
    }
}

function handleUploadChange(data: { fileList: UploadFileInfo[] }) {
    fileList.value = data.fileList;
}
function handleFileListChange() {
    if (fileList.value[0]) {
        formValue.value.cover = fileList.value[0].url as string;
    } else {
        formValue.value.cover = "";
    }
}
function handleRemoveFile(options: { file: UploadFileInfo; fileList: Array<UploadFileInfo> }): Promise<boolean> | boolean | any {
    formValue.value.cover = "";
    if (options.file.status != "finished") {
        removeImg.value = parseInt(`${options.file.id}`);
    }
}

async function onUploadEditorImg(files: File[], callback: (urls: string[]) => void) {
    const form = new FormData();
    files.forEach((file) => {
        form.append("file", file);
    });
    try {
        const { data } = await uploadFile(form, {
            withCredentials: true,
            headers: {
                "Content-Type": "multipart/form-data",
            },
        })
        callback(
            (data as any[]).map((file) => `${import.meta.env.VITE_API_URL}/file/${file.id}`)
        );
    }
    catch (error) {
        message.error((error as AxiosError).message);
    }

}
</script>
