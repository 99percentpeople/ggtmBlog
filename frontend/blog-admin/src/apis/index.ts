import { FileQuery } from "./../models/index";
import axios, { AxiosRequestConfig } from "axios";
import { router } from "@/routers";
import { SortQuery, TagQuery, BlogQuery, BlogModel } from "@/models";
// const { data, isFinished } = useAxios("/posts");

export const instance = axios.create({
    baseURL: import.meta.env.VITE_API_URL,
    withCredentials: true,
});

// 使用 Token 进行验证
// instance.interceptors.request.use(
//     async (request) => {
//         const userStore = useUserStore();
//         if (request.headers && userStore.userToken) {
//             request.headers.Authorization = userStore.userToken;
//         }
//         return request;
//     },
//     async (error) => {
//         throw error;
//     }
// );
instance.interceptors.response.use(
    (response) => {
        return response;
    },
    (error) => {
        if (error && error.response.status) {
            switch (error.response.status) {
                case 400:
                    error.message = "请求出错";
                    break;
                case 401:
                    error.message = "授权失败，请重新登录";
                    router.replace({ name: "login" });
                    break;
                case 403:
                    error.message = "拒绝访问，权限不足";
                    break;
                case 404:
                    error.message = "未找到该资源";
                    break;
            }
        } else if (!error.message) {
            error.message = "连接服务器失败";
        }
        if (error.response.data) {
            error.message += `: ${error.response.data}`;
        }
        throw error;
    }
);

export async function getBlogList(size: number, index: number, params?: BlogQuery) {
    const res = await instance.get(`/admin/blog/${size}/${index - 1}`, { params });
    return res.data;
}

export async function getBlog(blog_id: number) {
    const res = await instance.get(`/admin/blog/${blog_id}`);
    return res.data;
}

export async function getSortList(size: number, index: number, query?: SortQuery) {
    const res = await instance.get(`/sort/${size}/${index - 1}`, { params: query });
    return res.data;
}

export async function postOneSort(name: string) {
    const res = await instance.post("/sort", { name });
    return res.data;
}

export async function putSort(id: number, name: string) {
    const res = await instance.put(`/sort/${id}`, { name });
    return res.data;
}

export async function deleteSort(id: number) {
    const res = await instance.delete(`/sort/${id}`);
    return res.data;
}

export async function getTagsList(size: number, index: number, query?: TagQuery) {
    const res = await instance.get(`/tag/${size}/${index - 1}`, { params: query });
    return res.data;
}

export async function postOneTag(name: string) {
    const res = await instance.post("/tag", { name });
    return res.data;
}

export async function putTag(id: number, name: string) {
    const res = await instance.put(`/tag/${id}`, { name });
    return res.data;
}

export async function deleteTag(id: number) {
    const res = await instance.delete(`/tag/${id}`);
    return res.data;
}

export async function postOneBlog(blog: BlogModel) {
    const res = await instance.post("/admin/blog", blog);
    return res.data;
}

export async function deleteBlog(blog_id: number) {
    const res = await instance.delete(`/admin/blog/${blog_id}`);
    return res.data;
}

export async function putBlog(blog_id: number, blog: BlogModel) {
    const res = await instance.put(`/admin/blog/${blog_id}`, blog);
    return res.data;
}

export async function uploadFile(data: FormData, config?: AxiosRequestConfig) {
    const res = await instance.post("/file", data, config);
    return res.data;
}

export async function deleteFile(file_id: number) {
    const res = await instance.delete(`/file/${file_id}`);
    return res.data;
}

export async function putFile(file_id: number, data: FormData, config?: AxiosRequestConfig) {
    const res = await instance.put(`/${file_id}`, data, config);
    return res.data;
}

export async function getUserFiles(size: number, index: number, fileQuery?: FileQuery) {
    const res = await instance.get(`/file/info/${size}/${index - 1}`, { params: fileQuery });
    return res.data;
}
