<template>
  <div class="demo-pagination-block">
    <el-pagination
        v-model:currentPage="currentPage1"
        v-model:page-size="pageSize"
        :page-sizes="[3, 5, 7, 10]"
        small
        layout="sizes, prev, pager, next"
        :total="imageStore.getTagsTotal(image_name)"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
    />
  </div>
</template>

<script>
import {ref} from "vue";
import {useImageStore} from "@/store/image_table";

export default {
  name: "TagPagination",
  props:['name'],
  setup(props){
    // eslint-disable-next-line vue/no-setup-props-destructure
    const image_name=props.name
    const imageStore = useImageStore();

    const currentPage1=ref(1)
    const pageSize = ref(2)
    const handleSizeChange = (val) => {
      imageStore.UpdateTags(props.name,val,0)
    }
    const handleCurrentChange = (val) => {
      imageStore.UpdateTags(props.name,pageSize.value,(currentPage1.value-1)*pageSize.value)
    }
    return{
      // eslint-disable-next-line vue/no-dupe-keys
      currentPage1,pageSize,imageStore,handleSizeChange,handleCurrentChange,image_name
    }

  }
}
</script>

<style scoped>

</style>