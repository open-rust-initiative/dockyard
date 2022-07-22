import {createI18n} from "vue-i18n";
import zhCn from "./lang/zh-cn";
import en from "./lang/en";

let language = (navigator.language || "en"
).toLowerCase();
language = (language == "zh-cn" || language == "zh") ? "zhCn" : language
const i18n = createI18n({
    fallbackLocale: 'en',
    locale: sessionStorage.getItem("locale_lang") || language,
    messages: {
        zhCn,
        en,
    }
})
export default i18n;