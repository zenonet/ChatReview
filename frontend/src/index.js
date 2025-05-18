import { createApp } from 'vue';
import App from './App.vue';
import './index.css';
import { createMemoryHistory, createRouter, RouteRecordRaw } from 'vue-router';
import LoginView from './pages/LoginView.vue';
import ChatEditor from './pages/ChatEditor.vue';
import Root from './root.vue';
import router from './routes'





createApp(Root)
    .use(router)
    .mount('#root');
