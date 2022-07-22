<template>
  <el-dialog v-model="dialogVisible" width="30%" draggable center :title="$t('message.user_profile')">
    <el-form :model="change_user_param"
             status-icon
             :rules="{
              email: [
                  {required: true,message: $t('message.email_input'),trigger: 'blur',},
                  {pattern:/^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/,message: $t('message.email_check'),trigger: 'blur'}
                  ],
              name: [
                  {required: true,message: $t('message.name_input'),trigger: 'blur',},
                  {max:20,message:$t('message.name_check'),trigger: 'blur'},
                  ],
              comment: [
                  {required: false, trigger: 'blur'}]
        }"
             ref="change_user_ruleFormRef"
             label-width="0px" class="ms-content">
      <el-form-item prop="username">
        <el-input
            :placeholder="userStore.username"
            disabled
            class="w-50 m-2"
            size="large"
            :prefix-icon="User"/>
      </el-form-item>
      <el-form-item prop="email">
        <el-input
            v-model="change_user_param.email"
            :placeholder="userStore.email"
            class="w-50 m-2"
            size="large"
            :prefix-icon="House"/>
      </el-form-item>
      <el-form-item prop="name">
        <el-input
            v-model="change_user_param.name"
            :placeholder="userStore.name"
            class="w-50 m-2"
            size="large"
            :prefix-icon="UserFilled"/>
      </el-form-item>
      <el-form-item prop="comment">
        <el-input
            v-model="change_user_param.comment"
            :placeholder="userStore.comment"
            class="w-50 m-2"
            size="large"
            :prefix-icon="Comment"/>
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{ $t('message.cancel') }}</el-button>
        <el-button type="primary" @click="submit_change_Form(change_user_ruleFormRef)" :disabled="disregister"
        >{{ $t('message.confirm') }}</el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script lang="ts" setup>
import {computed, reactive, ref} from "vue";
import emitter from "../main";
import {FormInstance} from "element-plus/es";
import {useUserStore} from "../store/user";
import {Comment, House, User, UserFilled} from '@element-plus/icons-vue'
import {ElMessage} from "element-plus";

const dialogVisible = ref(false)
emitter.on("change_user_profile", (dialog) => {
  typeof dialog === "boolean" ? dialogVisible.value = dialog : false;
})
const change_user_ruleFormRef = ref<FormInstance>()
const change_user_param = reactive({
  email: "",
  name: "",
  comment: ''
});
let userStore = useUserStore();
change_user_param.email = userStore.email;
change_user_param.name = userStore.name;
change_user_param.comment = userStore.comment;
const disregister = computed(
    () => {
      return !(change_user_param.email != ""
          && change_user_param.name != "");
    }
)
const activeName = ref('first')
const submit_change_Form = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.validate(async (valid) => {
        if (valid) {
          let result = await userStore.UserUpdate({
            username: userStore.username,
            name: change_user_param.name,
            email: change_user_param.email,
            comment: change_user_param.comment
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