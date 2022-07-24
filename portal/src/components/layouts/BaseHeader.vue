<template>

  <el-menu class="el-menu-demo" mode="horizontal" :ellipsis="false">
    <div id="logo" @click="Side_is_Collapse">
      <img src="@/assets/dockyard.jpeg" style=" margin-left:20px;height: 50px;width: 70px" >
    </div>
    <div class="flex-grow"/>
    <el-sub-menu index="1">
      <template #title>{{ locale_label }}</template>
      <el-menu-item v-for="item in languages_options" :key="item.value" :index=item.index @click="select_language(item.label,item.value)">
        {{ item.label }}
      </el-menu-item>
    </el-sub-menu>
    <el-menu-item h="full" @click="toggleDark();changeicon()" index="2">
      <svg class="iconitem" aria-hidden="true" style="height: 56px;" v-show="!isdark">
        <use xlink:href="#icon-heianmoshi"></use>
      </svg>
      <svg class="iconitem" aria-hidden="true" style="height: 56px;color:white " v-show="isdark">
        <use xlink:href="#icon-baitianmoshi"></use>
      </svg>
    </el-menu-item>
    <el-sub-menu index="3">
      <template #title>
        <svg class="iconitem" aria-hidden="true" style="height: 56px;">
          <use xlink:href="#icon-user-filling"></use>
        </svg>
      </template>
      <el-menu-item index="3-1" @click="change_user">
        {{ $t('message.user_profile') }}
      </el-menu-item>
      <el-menu-item index="3-2" @click="change_password">
        {{ $t('message.change_password') }}
      </el-menu-item>
      <el-menu-item index="3-3" @click="Login_out">
        {{ $t('message.log_out') }}
      </el-menu-item>
    </el-sub-menu>
    <UserProfile></UserProfile>
    <ChangePasswd></ChangePasswd>
  </el-menu>
</template>
<script setup>
import { useDark, useToggle } from '@vueuse/core'
import emitter from "@/main";
import {computed, ref} from "vue";
import {useI18n} from "vue-i18n";
import {useBaseStore} from "@/store/base";
import {useUserStore} from "@/store/user";
import router from "@/router";
const isDark = useDark()
const toggleDark = useToggle(isDark)
const isdark=ref(false)
function changeicon (){

  isdark.value=!isdark.value
  console.log("edark",isdark.value)
}




let side_is_collapse = false;

function Side_is_Collapse() {
  side_is_collapse = !side_is_collapse;
  emitter.emit("side_is_collapse", side_is_collapse)
}

const value = ref('')

const baseStore = useBaseStore()
const userStore = useUserStore();
const locale = computed(() => baseStore.locale)
const locale_label = computed(() => baseStore.locale_label)
const locale_i18n = useI18n();

function select_language(lan_label, lan_value) {
  locale_i18n.locale.value = lan_value;
  baseStore.updateLocale(lan_value)
  baseStore.updateLocaleLabel(lan_label)
}

function change_user() {
  emitter.emit("change_user_profile", true)
}

function change_password() {
  emitter.emit("change_password", true)
}

function Login_out() {
  userStore.UserLoginOut()
  router.push("/login")
}

const languages_options = [
  {
    value: 'en',
    index: "1-1",
    label: "English",
  },
  {
    value: 'zhCn',
    index: "1-2",
    label: "简体中文",
  },
]
</script>
<style scoped>
#logo {
  height: 100%;
  padding: 0 20px;
  display: flex;
  text-align: center;
  justify-content: center;
  align-content: center;
}

#logo img {
  width: 5%;
}

.iconitem {
  width: 2em;
  height: 2em;
  vertical-align: -0.15em;
  fill: currentColor;
  overflow: hidden;
}
.flex-grow {
  flex-grow: 1;
}
</style>
