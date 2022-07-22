<template>
  <el-dialog v-model="dialogVisible" width="30%" draggable center :title="$t('message.change_password')">
    <el-form :model="change_passwd_param"
             status-icon
             :rules="{
              passwd: [
                  {required: true, validator: validatePass, message: $t('message.CurrentPassword_input'), trigger: 'blur'},
                  {max:30,message: $t('message.password_check'),trigger: 'blur'}],
              NewPasswd:[
                  {required: true, validator: validatePass1, message: $t('message.NewPassword_input'), trigger: 'blur'},
                  {max:30,message: $t('message.password_check'),trigger: 'blur'}],
              checkNewPass: [
                  {required: true, validator: validatePass2, message:$t('message.CheckNewPassword_match'),trigger: 'blur'}],
        }"
             ref="change_passwd_ruleFormRef"
             label-width="0px" class="ms-content">
      <el-form-item prop="passwd">
        <el-input
            v-model="change_passwd_param.passwd"
            :placeholder="$t('message.CurrentPassword')"
            type="password"
            class="w-50 m-2"
            size="large"
            :prefix-icon="Unlock"
        />
      </el-form-item>
      <el-form-item prop="NewPasswd">
        <el-input
            v-model="change_passwd_param.NewPasswd"
            :placeholder="$t('message.NewPassword')"
            type="password"
            class="w-50 m-2"
            size="large"
            :prefix-icon="Lock"
        />
      </el-form-item>
      <el-form-item prop="checkNewPass">
        <el-input
            v-model="change_passwd_param.checkNewPass"
            :placeholder="$t('message.CheckNewPassword')"
            type="password"
            class="w-50 m-2"
            size="large"
            :prefix-icon="Lock"
        />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{ $t('message.cancel') }}</el-button>
        <el-button type="primary" @click="submit_change_Form(change_passwd_ruleFormRef)" :disabled="disupdate"
        >{{ $t('message.confirm') }}</el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script lang="ts" setup>
import {computed, reactive, ref} from "vue";
import emitter from "../main";
import {FormInstance} from "element-plus/es";
import {useUserStore} from "@/store/user";
import {ElMessage} from "element-plus";
import {Lock, Unlock} from '@element-plus/icons-vue'
import {sha256} from "js-sha256";

const dialogVisible = ref(false)
emitter.on("change_password", (dialog) => {
  typeof dialog === "boolean" ? dialogVisible.value = dialog : false;
})
const change_passwd_ruleFormRef = ref<FormInstance>()
const change_passwd_param = reactive({
  passwd: "",
  NewPasswd: "",
  checkNewPass: ""
});
let userStore = useUserStore();
const validatePass = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('Please input the password'))
  } else {
    if (change_passwd_param.checkNewPass != '' && change_passwd_param.NewPasswd != '') {
      if (!change_passwd_ruleFormRef.value) return
      change_passwd_ruleFormRef.value.validateField('checkNewPass', () => null)
    }
    callback()
  }
}
const validatePass1 = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('Please input the password'))
  } else {
    if (change_passwd_param.checkNewPass != '' && change_passwd_param.passwd != '') {
      if (!change_passwd_ruleFormRef.value) return
      change_passwd_ruleFormRef.value.validateField('checkNewPass', () => null)
    }
    callback()
  }
}
var validatePass2 = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('please confirm new password'))
  } else if (value !== change_passwd_param.NewPasswd) {
    callback(new Error("new password do not match"))
  } else {
    callback()
  }
}
const disupdate = computed(
    () => {
      return !(change_passwd_param.passwd != ""
          && change_passwd_param.NewPasswd != ""
          && change_passwd_param.checkNewPass != "");
    }
)
const submit_change_Form = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.validate(async (valid) => {
        if (valid) {
          let result = await userStore.UserPasswdUpdate({
            username: userStore.username,
            password: sha256(userStore.username + sha256(change_passwd_param.passwd)),
            new_password: sha256(userStore.username + sha256(change_passwd_param.NewPasswd))
          });
          if (result) {
            dialogVisible.value = false;
            ElMessage.success("Success");
            return true;
          }
        }
        ElMessage.error("Failure");
        return false
      }
  )
}
</script>

<style scoped>
.ms-content {
  padding: 10px 20px 0px 20px;
}

</style>