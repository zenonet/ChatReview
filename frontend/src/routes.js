import { createMemoryHistory, createRouter } from "vue-router"
import App from "./App.vue"
import LoginView from "./components/LoginView.vue"

const routes/* :RouteRecordRaw[] */ = [
    { path: "/", component: App },
    { path: "/login", component: LoginView },
    { path: "/edit", component: LoginView }
]

const router = createRouter({
    history: createMemoryHistory(),
    routes
})

export default router;