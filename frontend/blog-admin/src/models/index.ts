export type UserInfo = {
    id: number;
    nickname?: string;
    username: string;
    email?: string;
    avatar?: string;
    create_time: Date;
    access_level: number
};

export type BlogModel = {
    title: string;
    content: string;
    cover?: string | null;
    flag: "Original" | "Reprint" | "Translate";
    summary: string;
    appreciation: boolean;
    share_statement: boolean;
    enable_comment: boolean;
    published: boolean;
    recommend: boolean;
    sort_id: number;
    user_id: number;
    tag_ids: number[];
};

export type BlogDetail = {
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
    sort?: Sort;
    published?: boolean;
    user: UserInfo;
    tags: Tag[];
};

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
    title?: string | null;
    sort_id?: number | null;
    order?: "asc" | "desc" | null;
    sort_by?: "update_time" | "create_time" | null;
    recommend?: string | number | boolean;
    summary?: boolean | null;
    cover?: boolean | null;
    tag_ids?: string | null;
};

export type FileQuery = {
    name?: string| null;
}