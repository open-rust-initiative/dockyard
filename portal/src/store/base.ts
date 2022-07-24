import {defineStore} from "pinia";

export const useBaseStore = defineStore("base", {
    state: () => {
        return {
            locale: sessionStorage.getItem("locale_lang") || "zhCn",
            locale_label: sessionStorage.getItem("locale_lang_label") || "简体中文"
        }
    },
    actions: {
        updateLocale(locale: string) {
            this.locale = locale
        },
        updateLocaleLabel(locale_label: string) {
            this.locale_label = locale_label
        }
    }
})