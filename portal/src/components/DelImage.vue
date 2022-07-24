<template>
  <el-dialog v-model="dialogVisible" width="20%" draggable center>
    <h2>{{$t('message.ImageName')}} : {{info.name}}</h2>
    <h3>{{$t('message.TagsCount')}} : {{info.tags_count}}</h3>
    <h3>{{$t('message.ImageSizeNoMB')}} : {{info.size}} (MiB)</h3>
    <h3>{{$t('message.ImageFirstCreated')}} : {{info.first_creation}}</h3>
    <h3>{{$t('message.ImageLastCredated')}} : {{info.last_creation}}</h3>
    <h3>{{$t('message.ImageLastPull')}} : {{info.last_pull}}</h3>
    <h3>{{$t('message.ImageLastPush')}} : {{info.last_push}}</h3>
  <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{ $t('message.cancel') }}</el-button>
        <el-button type="primary" @click="submit_del_image(info.name)" >{{ $t('message.remove') }}</el-button>
      </span>
  </template>
  </el-dialog>
</template>

<script setup>
import {reactive, ref} from "vue";
import emitter from "@/main";
import {ElMessage} from "element-plus";
import {useImageStore} from "@/store/image_table";
const imageStore = useImageStore();
const dialogVisible = ref(false)
let info=reactive({})
emitter.on("delete_image", (row) => {
  dialogVisible.value=true;
  info.name=row.name
  info.size=row.size
  info.tags_count=row.tags_count
  info.first_creation=row.timeinfo.first_creation;
  info.last_creation=row.timeinfo.last_creation;
  info.last_push=row.timeinfo.last_push;
  info.last_pull=row.timeinfo.last_pull;
})
const submit_del_image=async (name) => {
  console.log("name--",name);
  imageStore.DelImage(name).then(()=>{
    dialogVisible.value=false;
    ElMessage.success("Delete Success");
  }).catch(()=>{
    dialogVisible.value=false;
    ElMessage.error("Delete Failure");
  })
}
</script>

<style scoped>

</style>