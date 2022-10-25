<template>
    <n-card
        header-style="padding:6px 12px"
        content-style="padding:6px;"
        footer-style="padding:6px"
        :bordered="false"
        hoverable
    >
        <n-space :wrap="false">
            <n-avatar size="large" round :src="model.comment.avatar" v-if="model.comment.avatar"/>
            <n-avatar size="large" round v-else>{{ model.comment.nickname }}</n-avatar>
            <n-space vertical class="comment">
                <n-text depth="2">{{ model.comment.nickname }}</n-text>
                <n-text>
                    {{ model.comment.content }}
                </n-text>
                <n-space>
                    <n-text depth="3">
                        <small>{{ model.comment.create_time?.toLocaleDateString() }}</small>
                    </n-text>
                    <n-button text size="tiny" tag="a" @click="onReply(model.comment)">回复</n-button>
                </n-space>
                <sub-visitor-comment v-for="reply in replys" :reply="reply" @reply="onReply"></sub-visitor-comment>
                <n-button v-if="showFold" text @click="toggle()" size="tiny">{{
                    value ? `共 ${model.sub_comments.length} 条回复 点击展开` : "收起"
                }}</n-button>
            </n-space>
        </n-space>
    </n-card>
</template>

<script setup lang="ts">
import { CommentDetail, CommentModel } from "common/models";

const props = defineProps<{
    model: CommentDetail;
    loading?: boolean;
}>();
const showFold = ref(props.model.sub_comments.length > 3);

const [value, toggle] = useToggle(true);

function onReply(model: CommentModel) {
    emits("reply", model);
}
const replys = computed(() => {
    let re = props.model.sub_comments.map((item) => {
        if (item.id_ref != props.model.comment.id) {
            return {
                from: item,
                to: props.model.sub_comments[props.model.sub_comments.findIndex((e) => e.id == item.id_ref)],
            };
        } else {
            return {
                from: item,
                to: props.model.comment,
            };
        }
    });
    if (re.length > 3 && showFold.value && value.value) {
        return re.slice(0, 3);
    }
    return re;
});
const emits = defineEmits<{
    (event: "reply", reply_to: CommentModel): void;
}>();
</script>

<style scoped lang="scss">
.comment{
    word-break: break-all;
}
</style>