import FocusView from "../views/FocusView.vue";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/focus",
        name: "Focus",
        component: FocusView,
        props: true,
    },
];
const router = createRouter({
    history: createWebHistory("/"),
    routes,
});

export default router;
