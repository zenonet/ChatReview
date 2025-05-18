import { createMemoryHistory, createRouter, createWebHistory } from "vue-router"
import App from "./App.vue"
import LoginView from "./components/LoginView.vue"
import ChatEditor from "./components/ChatEditor.vue"
import ChatCreator from "./components/ChatCreator.vue"

const routes/* :RouteRecordRaw[] */ = [
    { path: "/", component: App },
    { path: "/login", component: LoginView },
    { path: "/edit/:id", component: ChatEditor },
    { path: "/newchat", component: ChatCreator }
]

const router = createRouter({
    history:  createWebHistory(),
    routes
})

export default router;