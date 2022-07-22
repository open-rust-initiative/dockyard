import {defineStore} from "pinia";
import {reqLogin, reqRegister, reqUpdateUser, reqUpdateUserPasswd} from "@/api";

export const useUserStore = defineStore("user", {
    state: () => {
        return {
            username: localStorage.getItem("username") || "",
            name: localStorage.getItem("name") || "",
            email: localStorage.getItem("email") || "",
            admin: false,
            comment: localStorage.getItem("comment") || "",
            token: localStorage.getItem("token") || ""
        }
    },
    actions: {
        updateUserName(username: string) {
            this.username = username
        },
        updateName(name: string) {
            this.name = name;
        },
        updateAdmin(admin: boolean) {
            this.admin = admin;
        },
        updateEmail(email: string) {
            this.email = email;
        },
        updateToken(token: string) {
            this.token = token;
        },
        updateComment(comment: string) {
            this.comment = comment;
        },
        async UserLogin(info: any) {
            const res = await reqLogin(info);
            if (res.status != 200) {
                return false
            } else {
                const data = res.data;
                this.updateUserName(data.username);
                localStorage.setItem("username",data.username);
                this.updateName(data.name);
                localStorage.setItem("name",data.name);
                this.updateAdmin(data.admin);
                this.updateEmail(data.email);
                localStorage.setItem("email",data.email);
                this.updateToken(data.token);
                localStorage.setItem("token",data.token);
                this.updateComment(data.comment);
                return true
            }
        },
        async UserRegister(info: any) {
            const res = await reqRegister(info);
            if (res.status != 200) {
                return false
            } else {

                const data = res.data;
                console.log('user register',data)
                this.updateUserName(info.username)
                this.updateToken(data.token)
                this.updateName(info.name)
                this.updateEmail(info.email)
                this.updateComment(info.comment)
                return true
            }
        },
        async UserUpdate(info: any) {
            const res = await reqUpdateUser(info);
            if (res.status != 200) {
                return false
            } else {
                this.updateName(info.name);
                this.updateEmail(info.email);
                this.updateComment(info.comment);
                return true
            }
        },
        async UserPasswdUpdate(info: any) {
            const res = await reqUpdateUserPasswd(info);
            if (res.status != 200) {
                return false
            } else {
                return true;
            }
        },
        async UserLoginOut() {
            localStorage.removeItem("username");
            this.updateUserName("");
            localStorage.removeItem("name");
            this.updateName("");
            localStorage.removeItem("email");
            this.updateEmail("");
            localStorage.removeItem("comment");
            this.updateComment("");
            localStorage.removeItem("token");
            this.updateToken("");
            this.updateAdmin(false);
        },

    }
})