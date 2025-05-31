import { createRouter, createWebHashHistory, createWebHistory, RouteRecordRaw } from "vue-router"
import App from "./App.vue"
import LoginView from "./pages/LoginView.vue"
import ChatEditor from "./pages/ChatEditor.vue"
import ChatCreator from "./pages/ChatCreator.vue"
import NotFound from "./NotFound.vue"
import { compile } from "vue"
import MyChats from "./pages/MyChats.vue"
import MyChatsAsync from "./pages/MyChatsAsync.vue"
import Profile from "./pages/Profile.vue"
import NewLayout from "./pages/NewLayout.vue"
import PlatformStatistics from "./components/PlatformStatistics.vue"
import LandingPage from "./pages/LandingPage.vue"
import Feed from "./pages/Feed.vue"
import ChatViewerById from "./pages/ChatViewerById.vue"
import AddPasskey from "./pages/AddPasskey.vue"

const routes: RouteRecordRaw[] = [
    { path: "/", component: LandingPage },
    { path: "/login", component: LoginView },
    { path: "/edit/:id", component: ChatEditor},
    { path: "/chat/:id", component: ChatViewerById },
    { path: "/newchat", component: ChatCreator },
    { path: "/mychats", component: MyChatsAsync},
    { path: "/profile", component: Profile },
    { path: "/passkeys", component: AddPasskey },

    { path: "/feed", component: Feed },

    { path: "/stats", component: PlatformStatistics },

    { path: "/newlayout", component: NewLayout },

    { path: "/:pathMatch(.*)*", component: NotFound }
]

const router = createRouter({
    history:  createWebHashHistory(),
    routes,
})

export default router;