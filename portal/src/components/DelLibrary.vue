<template>
  <el-dialog v-model="dialogVisible" width="20%" draggable center>
    <h2>{{$t('message.LibraryName')}} : {{info.name}}</h2>
    <h3>{{$t('message.LibrarySize')}} : {{info.size}} (MiB)</h3>
    <h3>{{$t('message.RepCount')}} : {{info.images_count}}</h3>
    <h3>{{$t('message.TagCount')}} : {{info.tags_count}}</h3>
  <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">{{ $t('message.cancel') }}</el-button>
        <el-button type="primary" @click="submit_del_library(info.name)" >{{ $t('message.remove') }}</el-button>
      </span>
  </template>
  </el-dialog>
</template>

<script setup>
import {reactive, ref} from "vue";
import emitter from "@/main";
import {ElMessage} from "element-plus";
import {useLibraryStore} from "@/store/library_table";
let libraryStore = useLibraryStore();
const dialogVisible = ref(false)
let info=reactive({})
emitter.on("delete_library", (library_info) => {
  dialogVisible.value=true;
  info.name=library_info.name
  info.size=library_info.size
  info.images_count=library_info.images_count
  info.tags_count=library_info.tags_count;

})
const submit_del_library=async (name) => {
  libraryStore.DelLibrary(name).then(()=>{
    dialogVisible.value=false;
    ElMessage.success("Delete Success");
    emitter.emit("update_charts")
  }).catch(()=>{
    dialogVisible.value=false;
    ElMessage.error("Delete Failure");
  })
}
</script>

<style scoped>

</style>