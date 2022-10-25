// To parse this data:
//
//   import { Convert, BlogListItem } from "./file";
//
//   const blogListItem = Convert.toBlogListItem(json);

export interface BlogListItem {
    data: Data;
    msg: string;
}

export interface Data {
    items: number;
    list: List[];
    pages: number;
}

export interface List {
    cover: string;
    createTime: Date;
    flag: string;
    id: number;
    published: boolean;
    recommend: boolean;
    sort: Sort;
    summary: string;
    tags: Sort[];
    title: string;
    updateTime: null;
    user: User;
    views: number;
}

export interface Sort {
    blogCount: number;
    id: number;
    name: string;
}

export interface User {
    avatar: null;
    id: number;
    nickname: null;
    username: string;
}

// Converts JSON strings to/from your types
export class Convert {
    public static toBlogListItem(json: string): BlogListItem {
        return JSON.parse(json);
    }

    public static blogListItemToJson(value: BlogListItem): string {
        return JSON.stringify(value);
    }
}
