import {Expand,ExpandRecursively} from "../utils"
export type BlogDetailModel = {
    id: number;
    title: string;
    content?: string;
    cover?: string | null;
    flag: "Original" | "Reprint" | "Translate";
    summary?: string;
    appreciation?: boolean;
    views: number;
    share_statement: boolean;
    enable_comment: boolean;
    recommend: boolean;
    create_time?: Date | null;
    update_time?: Date | null;
    published?: boolean;
    sort?: Sort;
    user: UserInfo;
    tags: Tag[];
};
export type BlogSearch = Pick<BlogDetailModel, "id" | "title">

export type BlogListItem = Expand<Omit<BlogDetailModel,
    "content"
    | "appreciation"
    | "enable_comment"
    | "share_statement"
    | "user"> & { user: UserBriefInfo }>


export type UserInfo = {
    id: number;
    nickname?: string;
    username: string;
    email?: string;
    avatar?: string;
    create_time: Date;
};
export type UserBriefInfo = Pick<UserInfo, "id" | "nickname" | "username">

export type Sort = {
    id: number;
    name: string;
    blog_count: number;
};

export type Tag = {
    id: number;
    name: string;
    blog_count: number;
};

export type SortQuery = {
    name?: string;
    published?: boolean;
};

export type TagQuery = {
    name?: string;
    published?: boolean;
};

export type BlogQuery = {
    title?: string;
    sort_id?: number;
    tag_ids?: string;
    order?: "asc" | "desc";
    sort_by?: "update_time" | "create_time";
    recommend?: boolean;
    summary?: boolean;
    cover?: boolean;
    title_or_content?: string;
};



export type CommentModel = {
    id?: number | null;
    id_ref?: number | null;
    nickname?: string | null;
    email?: string | null;
    content: string | null;
    avatar?: string | null;
    like?: number | null;
    dislikes?: number | null;
    create_time?: Date | null;
    blog_id: number;
};

export type CommentDetail = {
    comment: CommentModel;
    sub_comments: CommentModel[];
};
export type List<T> = {
    msg: string;
    data: {
        items: number;
        pages: number;
        list: T[];
    };
};

export type PostResponse<T> = {
    msg: string;
    data : T
}
