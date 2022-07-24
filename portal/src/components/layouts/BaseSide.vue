<template>
  <el-menu
      default-active="1"
      class="el-menu-vertical-demo"
      :collapse="isCollapse"
      @open="handleOpen"
      @close="handleClose"
  >
    <el-menu-item index="1" @click="ToLibrary">
      <el-icon>
        <svg id="user_icon" aria-hidden="true" style="height: 56px;">
          <use xlink:href="#icon-structure"></use>
        </svg>
      </el-icon>
      <template #title>{{$t("message.Library")}}</template>
    </el-menu-item>
    <el-menu-item index="2">
      <el-icon>
        <setting/>
      </el-icon>
      <template #title>{{$t("message.SystemManage")}}</template>
    </el-menu-item>
  </el-menu>
</template>

<script lang="ts" setup>
import {ref} from 'vue'
import {Setting,} from '@element-plus/icons-vue'
import emitter from "@/main";
import {useRouter} from "vue-router";
let router = useRouter();
function ToLibrary(){
  router.push("/library")
}
const isCollapse = ref(false)
emitter.on("side_is_collapse", (collapse) => {
  typeof collapse === "boolean" ? isCollapse.value = collapse : false;
})
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
</script>

