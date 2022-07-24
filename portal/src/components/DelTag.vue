<template>
  <el-dialog v-model="dialogTagVisible" width="20%" draggable center>
    <h2>{{$t('message.ImageName')}} : {{info.name}}</h2>
    <h2>{{$t('message.Tag')}} : {{info.tag}}</h2>
    <h3>{{$t('message.TagSizeNoMB')}} : {{info.size}} (MiB)</h3>
    <h3>{{$t('message.CreationTime')}} : {{info.create_time}}</h3>
    <h3>{{$t('message.PullTime')}} : {{info.pull_time}}</h3>
    <h3>{{$t('message.PushTime')}} : {{info.push_time}}</h3>
  <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{ $t('message.cancel') }}</el-button>
        <el-button type="primary" @click="submit_del_tag(info.name,info.tag)" >{{ $t('message.remove') }}</el-button>
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
const dialogTagVisible = ref(false)
let info=reactive({})
emitter.on("delete_tag", (data) => {
  dialogTagVisible.value=true;
  info.name=data.name
  info.tag=data.row.tag
  info.size=data.row.size
  info.create_time=data.row.create_time;
  info.pull_time=data.row.pull_time;
  info.push_time=data.row.push_time;
})
const submit_del_tag=async (name,tag) => {
  console.log("name--",name);
  imageStore.DelTag(name,tag).then(()=>{
    dialogTagVisible.value=false;
    ElMessage.success("Delete Success");
  }).catch(()=>{
    dialogTagVisible.value=false;
    ElMessage.error("Delete Failure");
  })
}
</script>

<style scoped>

</style>