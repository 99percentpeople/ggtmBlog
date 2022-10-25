<template>
    <n-card size="small" header-style="padding:0 12px" content-style="padding:0 6px" :bordered="false">
        <n-space vertical>
            <n-space align="start" class="comment">
                <n-avatar size="small" round :src="reply.to.avatar" v-if="reply.to.avatar"></n-avatar>
                <n-avatar size="small" round v-else>{{ reply.to.nickname }}</n-avatar>
                <n-text>{{ reply.from.nickname }}</n-text>
                <n-text :depth="3">{{ `回复 @${reply.to.nickname}: ` }}</n-text>
                <n-text>{{ reply.from.content }}</n-text>
            </n-space>
            <n-space>
                <n-text depth="3">
                    <small>{{ reply.from.create_time?.toLocaleDateString() }}</small>
                </n-text>
                <n-button text size="tiny" tag="a" @click="onReply">回复</n-button>
            </n-space>
        </n-space>
    </n-card>
</template>

<script setup lang="ts">
import { CommentDetail, CommentModel } from "common/models";

const props = defineProps<{
    reply: Reply;
    loading?: boolean;
}>();

type Reply = {
    from: CommentModel;
    to: CommentModel;
};

const emits = defineEmits<{
    (event: "reply", reply_to: CommentModel): void;
}>();

function onReply() {
    emits("reply", props.reply.from);
}
</script>

<style scoped lang="scss">
.comment{
    word-break: break-all;
}
</style>