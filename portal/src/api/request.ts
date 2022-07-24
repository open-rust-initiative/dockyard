import axios from "axios";
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import nprogress from 'nprogress'
import 'nprogress/nprogress.css'
import {useUserStore} from "@/store/user";
import router from "@/router";


const request = axios.create(
    {
        baseURL: "/ui",
        timeout: 5000,
    }
);
request.interceptors.request.use((config) => {
    const userStore = useUserStore();
    if (userStore.token) {
        config.headers.Authorization = 'Bearer ' + userStore.token;
    }
    nprogress.start();
    return config;
}, function (error) {
    return Promise.reject(error)
});
request.interceptors.response.use((res) => {
    nprogress.done();
    return res
}, (res_error) => {
    nprogress.done();
    if (res_error.response.status == 401) {
        console.log("token needed")
        const store = useUserStore();
        store.UserLoginOut();
        router.push("/login")
    }
    return Promise.reject(new Error("fail"));
})
export default request;
