import "vue-router";
declare module "vue-router" {
    interface RouteMeta {
        transition?: string;
        requireAuth?: boolean;
        AccessLevel?: number;
    }
}
