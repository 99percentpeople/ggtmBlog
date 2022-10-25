"use strict";
// To parse this data:
//
//   import { Convert, BlogListItem } from "./file";
//
//   const blogListItem = Convert.toBlogListItem(json);
Object.defineProperty(exports, "__esModule", { value: true });
exports.Convert = void 0;
// Converts JSON strings to/from your types
class Convert {
    static toBlogListItem(json) {
        return JSON.parse(json);
    }
    static blogListItemToJson(value) {
        return JSON.stringify(value);
    }
}
exports.Convert = Convert;
