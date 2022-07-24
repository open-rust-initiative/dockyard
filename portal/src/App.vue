<template>
  <el-config-provider namespace="ep" :locale="locale">
    <router-view/>
  </el-config-provider>
</template>
<script setup>
import {computed, provide} from "vue";
import zhCn from "element-plus/lib/locale/lang/zh-cn"
import en from "element-plus/lib/locale/lang/en"
import {useBaseStore} from "./store/base";

const baseStore = useBaseStore()
const language = computed(() => baseStore.locale)
const lang = {
  zhCn,
  en
}
const locale = computed(() => lang[language.value])
</script>
<style lang="scss">
@import "./assets/css/main.css";
@import "./assets/css/color-dark.css";
</style>
