import axios, { AxiosError, AxiosResponse } from "axios";
import { router } from "@/routers";
import { SortQuery, TagQuery, BlogDetailModel, BlogQuery, CommentModel, List, BlogSearch, Tag, Sort, CommentDetail, PostResponse } from "common/models";

export const instance = axios.create({
    baseURL: import.meta.env.VITE_API_URL,
    withCredentials: true,
});

instance.interceptors.response.use(
    async (response: AxiosResponse) => {
        return response;
    },
    async (error: AxiosError): Promise<AxiosError<string>> => {
        if (error && error.response?.status) {
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
        if (error.response?.data) {
            error.message += `: ${error.response.data}`;
        }
        throw error as AxiosError;
    }
);

export async function getBlog(blog_id: number) {
    const res = await instance.get(`/blog/${blog_id}`, {
        params: {
            view: true,
        },
    });
    return res.data;
}
export async function getBlogList(
    size: number,
    index: number,
    params: BlogQuery
): Promise<{
    data: {
        list: BlogDetailModel[];
        items: number;
        pages: number;
    };
    msg: string;
}> {
    const res = await instance.get(`/blog/${size}/${index - 1}`, { params });
    return res.data;
}
export async function getSortList(
    size: number,
    index: number,
    query: SortQuery
): Promise<List<Sort>> {
    const res = await instance.get(`/sort/${size}/${index - 1}`, { params: query });
    return res.data;
}

export async function getTagsList(
    size: number,
    index: number,
    query: TagQuery
): Promise<List<Tag>> {
    const res = await instance.get(`/tag/${size}/${index - 1}`, { params: query });
    return res.data;
}

export async function getBlogSearchResult(query: BlogQuery): Promise<BlogSearch[]> {
    const res = await instance.get("/blog/search", { params: query });
    return res.data;
}

export async function getComments(blogId: number, size: number, index: number): Promise<List<CommentDetail>> {
    const res = await instance.get(`/comment/${blogId}/${size}/${index - 1}`);
    return res.data;
}

export async function postComment(blogId: number, comment: CommentModel): Promise<PostResponse<CommentModel>> {
    const res = await instance.post(`/comment/${blogId}`, comment);
    return res.data;
}









