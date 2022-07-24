import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";
import {useUserStore} from "@/store/user";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/login",
        name: "Login",
        meta: {
            title: '登录'
        },
        component: () => import("../views/Login.vue")
    },
    {
        path: '/',
        redirect: "/library"
    },
    {
        path: "/home",
        name: "Home",
        component: () => import("../views/Home.vue"),
        children: [
            {
                path: "/library",
                name: "Library",
                component: () => import("../views/Home/LibraryList.vue")
            },
            {
                path: "/image",
                name: "Image",

                component: () => import("../views/Home/ImageList.vue")
            },

        ]
    }
]


const router = createRouter({
    history: createWebHistory(),
    routes
})


router.beforeEach((to,
                   from,
                   next) => {
    const userStore = useUserStore();
    const token = userStore.token;
    // console.log("to == ", to.name != "Login")
    if (!token && to.name != "Login") {
        next({name: "Login"})
    } else {
        next()
    }
})
export default router