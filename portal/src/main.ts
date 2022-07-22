import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import {createPinia} from "pinia";
import "element-plus/dist/index.css";
import "element-plus/theme-chalk/src/message.scss"
import "@/styles/index.scss"
import './assets/css/icon.css'
import mitt from "mitt";
import i18n from "@/i18n";
import './assets/iconfont';

createApp(App)
    .use(createPinia())
    .use(router)
    .use(i18n)
    .mount('#app')
const emitter=mitt()
export default emitter
