import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router"
import App from "./App.vue"
import LoginView from "./pages/LoginView.vue"
import ChatEditor from "./pages/ChatEditor.vue"
import ChatCreator from "./pages/ChatCreator.vue"
import NotFound from "./NotFound.vue"
import { compile } from "vue"

const routes: RouteRecordRaw[] = [
    { path: "/", component: App },
    { path: "/login", component: LoginView },
    { path: "/edit/:id", component: ChatEditor },
    { path: "/newchat", component: ChatCreator },


    { path: "/:pathMatch(.*)*", component: NotFound }
]

const router = createRouter({
    history:  createWebHistory(),
    routes,
})

export default router;