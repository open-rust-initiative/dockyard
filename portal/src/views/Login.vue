<template>
  <div class="login-wrap">
    <transition name="el-zoom-in-center">
      <div class="ms-login " v-show="login_show">
        <div class="ms-title">Dockyard</div>
        <el-form :model="login_param"
                 status-icon
                 :rules="{
                  username: [
                      {required: true,message: $t('message.username_input'),trigger: 'blur'},
                      {max:255,message:$t('message.username_check'),trigger: 'blur'},
                      ],
                  password: [
                      {required: true, message: $t('message.password_input'), trigger: 'blur'},
                      {max:30,message:$t('message.password_check'),trigger:'blur'}
                      ],
                  }"
                 ref="login_ruleFormRef" label-width="0px" class="ms-content">
          <el-form-item prop="username">
            <el-input
                v-model="login_param.username"
                :placeholder="$t('message.username')"
                class="w-50 m-2"
                size="large"
                :prefix-icon="User"/>
          </el-form-item>
          <el-form-item prop="password">
            <el-input
                v-model="login_param.password"
                :placeholder="$t('message.password')"
                type="password"
                class="w-50 m-2"
                size="large"
                :prefix-icon="Unlock"
                @keyup.enter="submit_LoginForm"
            />
          </el-form-item>
          <div class="login-btn">
            <el-button type="primary" @click="submit_LoginForm" :disabled="dislogin">
              {{ $t("message.login") }}
            </el-button>
            <div class="flex justify-space-between mb-4 flex-wrap gap-4">
              <el-button
                  type="info"
                  text
                  @click="change_show">{{ $t("message.register") }}
              </el-button>
            </div>
          </div>
        </el-form>
      </div>
    </transition>
    <transition name="el-zoom-in-center">
      <div class="ms-login " v-if="!login_show">
        <div class="ms-title">Dockyard</div>
        <el-form :model="register_param"
                 status-icon
                 :rules="{
              username: [
                  {required: true,message: $t('message.username_input'),trigger: 'blur',},
                  {max:255,message:$t('message.username_check'),trigger: 'blur'}
                  ],
              email: [
                  {required: true,message: $t('message.email_input'),trigger: 'blur',},
                  {pattern:/^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/,message: $t('message.email_check'),trigger: 'blur'}
                  ],
              name: [
                  {required: true,message: $t('message.name_input'),trigger: 'blur',},
                  {max:20,message:$t('message.name_check'),trigger: 'blur'},
                  ],
              password: [
                  {required: true, validator: validatePass, message: $t('message.password_input'), trigger: 'blur'},
                  {max:30,message: $t('message.password_check'),trigger: 'blur'}],
              checkPass: [
                  {required: true, validator: validatePass2, message:$t('message.checkPass_input'),trigger: 'blur'}],
              comment: [
                  {required: false, trigger: 'blur'}]
        }"
                 ref="register_ruleFormRef"
                 label-width="0px" class="ms-content">
          <el-form-item prop="username">
            <el-input
                v-model="register_param.username"
                :placeholder="$t('message.username')"
                class="w-50 m-2"
                size="large"
                :prefix-icon="User"/>
          </el-form-item>
          <el-form-item prop="email">
            <el-input
                v-model="register_param.email"
                :placeholder="$t('message.email')"
                class="w-50 m-2"
                size="large"
                :prefix-icon="House"/>
          </el-form-item>
          <el-form-item prop="name">
            <el-input
                v-model="register_param.name"
                :placeholder="$t('message.name')"
                class="w-50 m-2"
                size="large"
                :prefix-icon="UserFilled"/>
          </el-form-item>
          <el-form-item prop="password">
            <el-input
                v-model="register_param.password"
                :placeholder="$t('message.password')"
                type="password"
                class="w-50 m-2"
                size="large"
                autocomplete="off"
                :prefix-icon="Unlock"
            />
          </el-form-item>
          <el-form-item prop="checkPass">
            <el-input
                v-model="register_param.checkPass"
                :placeholder="$t('message.checkPass')"
                type="password"
                class="w-50 m-2"
                size="large"
                autocomplete="off"
                :prefix-icon="Unlock"
            />
          </el-form-item>
          <el-form-item prop="comment">
            <el-input
                v-model="register_param.comment"
                :placeholder="$t('message.comment')"
                class="w-50 m-2"
                size="large"
                :prefix-icon="Comment"/>
          </el-form-item>
          <div class="login-btn">
            <el-button type="primary" @click="submit_RegisterForm(register_ruleFormRef)" :disabled="disregister">
              {{ $t("message.register") }}
            </el-button>
            <div class="flex justify-space-between mb-4 flex-wrap gap-4">
              <el-button
                  type="info"
                  text
                  @click="change_show">{{ $t("message.login") }}
              </el-button>
            </div>
          </div>
        </el-form>
      </div>
    </transition>
  </div>
</template>

<script lang="ts" setup>
import {computed, reactive, ref} from "vue";
import {useRouter} from "vue-router";
import {useStore} from "vuex";
import type {FormInstance} from 'element-plus'
import {ElMessage} from "element-plus";
import {Comment, House, Unlock, User, UserFilled} from '@element-plus/icons-vue'
import {useI18n} from "vue-i18n";
import {sha256} from "js-sha256";
import {useUserStore} from "@/store/user";

const userStore = useUserStore();
const router = useRouter();
const store = useStore()
const i18n = useI18n()
const register_ruleFormRef = ref<FormInstance>()
const login_ruleFormRef = ref<FormInstance>()
const login_param = reactive({
  username: "",
  password: "",
});

const validatePass = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('Please input the password'))
  } else {
    if (register_param.checkPass !== '') {
      if (!register_ruleFormRef.value) return
      register_ruleFormRef.value.validateField('checkPass', () => null)
    }
    callback()
  }
}

var validatePass2 = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('please confirm password'))
  } else if (value !== register_param.password) {
    callback(new Error("Password do not match"))
  } else {
    callback()
  }
}

const register_param = reactive({
  username: "",
  email: "",
  name: "",
  password: "",
  checkPass: "",
  comment: ''
});


let dislogin = computed(
    () => {
      return !(login_param.password != "" && login_param.username != "");
    }
)
const disregister = computed(
    () => {
      return !(register_param.username != ""
          && register_param.email != ""
          && register_param.name != ""
          && register_param.password != ""
          && register_param.checkPass != "");
    }
)
const login_show = ref(true);

function change_show() {
  login_show.value = !login_show.value
}

const submit_RegisterForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.validate(async (valid) => {
        if (valid) {
          let password_encoded = sha256(register_param.username + sha256(register_param.password));
          let result = await userStore.UserRegister({
            username: register_param.username,
            password: password_encoded,
            email: register_param.email,
            name: register_param.name,
            comment: register_param.comment
          });
          if (result) {
              ElMessage.success("Success");
              router.push("/library")
              return true;
          }
        }
        ElMessage.success("Failure");
        return false
      }
  )
}
const submit_LoginForm = async () => {
  let result = await userStore.UserLogin({
    username: login_param.username,
    password: sha256(login_param.username + sha256(login_param.password)),
  });
  if (result) {
    ElMessage.success("Success");
    router.push("/library");
  } else {
    ElMessage.error("Failure");
  }
}

</script>

<style scoped>
.login-wrap {
  position: relative;
  width: 100%;
  height: 100%;
  background-repeat: no-repeat;
  background-image: url(../assets/img/background.jpeg);
  background-attachment: scroll;
  /*background-size: 100%;*/
  overflow: hidden;
}

.ms-title {
  width: 100%;
  line-height: 50px;
  text-align: center;
  font-size: 20px;
  color: #fff;
  border-bottom: 1px solid #ddd;
}

.ms-login {
  position: fixed;
  left: 50%;
  top: 30%;
  width: 350px;
  margin: -190px 0 0 -175px;
  border-radius: 5px;
  background: rgba(255, 255, 255, 0.3);
  overflow: auto;
}

.ms-content {
  padding: 40px 30px 0px 30px;
}

.login-btn {
  text-align: center;
}

.login-btn button {
  width: 100%;
  padding: 20px;
  height: 36px;
  margin-bottom: 10px;
}

</style>