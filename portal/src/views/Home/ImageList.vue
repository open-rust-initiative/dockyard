<template>
  <div>
    <el-row justify="space-between" gutter="40">
      <el-col :span="13">
        <el-table :data="imageStore.images"  style="width: 100%" max-height="800px" >
          <el-table-column type="expand" >
            <template #default="props">
              <div m="4">
                <p m="t-0 b-2">{{$t('message.ImageFirstCreated')}} : {{ props.row.timeinfo.first_creation }}</p>
                <p m="t-0 b-2">{{$t('message.ImageLastCredated')}} : {{ props.row.timeinfo.last_creation }}</p>
                <p m="t-0 b-2">{{$t('message.ImageLastPush')}} : {{ props.row.timeinfo.last_push }}</p>
                <p m="t-0 b-2">{{$t('message.ImageLastPull')}} : {{ props.row.timeinfo.last_pull}}</p>
                <h3>{{$t('message.Tag')}}</h3>
                <el-table :data="props.row.tags" max-height="300px">
                  <el-table-column :label="$t('message.Tag')" prop="tag" sortable/>
                  <el-table-column :label="$t('message.Size')" prop="size" sortable/>
                  <el-table-column :label="$t('message.CreationTime')" prop="create_time" sortable/>
                  <el-table-column :label="$t('message.PullTime')" prop="pull_time"  sortable/>
                  <el-table-column :label="$t('message.PushTime')" prop="push_time" sortable/>
                  <el-table-column fixed="right" :label="$t('message.Operation')" width="120">
                    <template #default="scope">
                      <el-button
                          link
                          type="primary"
                          size="small"
                          @click.prevent="deletetag(scope.row,props.row.name)"
                      >
                        {{$t('message.remove')}}
                      </el-button>
                    </template>
                  </el-table-column>
                </el-table>
              </div>
            </template>
          </el-table-column>
          <el-table-column :label="$t('message.ImageName')" prop="name" />
          <el-table-column :label="$t('message.ImageSize')" prop="size" />
          <el-table-column fixed="right" :label="$t('message.Operation')" width="120">
            <template #default="scope">
              <el-button
                  link
                  type="primary"
                  size="small"
                  @click.prevent="deleteimage(scope.row)"
              >
                {{$t('message.remove')}}
              </el-button>
            </template>
          </el-table-column>
        </el-table>
        <div class="demo-pagination-block" style="margin-top: 20px">
          <el-pagination
              v-model:currentPage="currentPage"
              v-model:page-size="pageSize"
              :page-sizes="[3, 5, 10, 20]"
              :disabled="disabled"
              :background="background"
              layout="sizes, prev, pager, next"
              :total="imageStore.images_total"
              @size-change="handleSizeChange"
              @current-change="handleCurrentChange"
          />
        </div>
      </el-col>
      <el-col :span="11">
        <el-card shadow="hover" style="height: 600px" >
          <div id="image_chart" style="width: 100%;height: 600px;"></div>
        </el-card>
      </el-col>
    </el-row>
  </div>
  <DelTag></DelTag>
  <DelImage></DelImage>
</template>

<script>
import * as echarts from "echarts"
import {
  TitleComponent,
  ToolboxComponent,
  TooltipComponent,
  LegendComponent
} from 'echarts/components';
import { PieChart } from 'echarts/charts';
import { LabelLayout } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import {onMounted, ref} from "vue";
import {useLibraryStore} from "@/store/library_table";
import emitter from "@/main";
import {generat_options} from "@/views/Home/utils";
import {useImageStore} from "@/store/image_table";
import {useRoute} from "vue-router/dist/vue-router";

echarts.use([
  TitleComponent,
  ToolboxComponent,
  TooltipComponent,
  LegendComponent,
  PieChart,
  CanvasRenderer,
  LabelLayout
]);
export default {
  name: "ImageList",
  setup() {
    let libraryStore = useLibraryStore();
    let imageStore = useImageStore();
    const route = useRoute();
    let myEcharts=""
    const currentPage = ref(1)
    const pageSize = ref(5)
    const background = ref(true)
    const disabled = ref(false)

    const handleSizeChange = (val) => {
      imageStore.GetImages(route.query.library,val,0).then(()=>{
        let generatOptions = generat_options(imageStore.getName,imageStore.getCharData);
        myEcharts.setOption(generatOptions);
      })
    }
    const handleCurrentChange = (val) => {
      imageStore.GetImages(route.query.library,pageSize.value,(currentPage.value-1)*pageSize.value).then(()=>{
        let generatOptions = generat_options(imageStore.getName,imageStore.getCharData);
        myEcharts.setOption(generatOptions);
      })
    }
    function deletetag(row,name){
      console.log("delete tag row name",row,name);
      emitter.emit("delete_tag",{row:row,name:name})
    }
    function deleteimage(row){
      emitter.emit("delete_image",row)
      console.log("delete image",row)
    }
    emitter.on("update_charts",()=>{
      libraryStore.GetLibraryTotal().then(()=>{
        let generatOptions = generat_options(libraryStore.getName,libraryStore.getCharData);
        myEcharts.setOption(generatOptions);
      })
    })
    onMounted(() => {
      imageStore.GetImageTotal(route.query.library)
      imageStore.GetImages(route.query.library,pageSize.value,null).then(()=>{
        myEcharts = echarts.init(document.getElementById("image_chart"))
        window.addEventListener("resize", () => {
          myEcharts.resize()
        })
        console.log("getName",imageStore.getName)
        console.log("charData",imageStore.getCharData)
        let generatOptions = generat_options(imageStore.getName,imageStore.getCharData);
        myEcharts.setOption(generatOptions);
      })
    })
      return {
        imageStore,myEcharts,disabled,currentPage,background,pageSize,handleCurrentChange,handleSizeChange,route,deletetag,deleteimage
      }
    }
  }

</script>

<style scoped>
.demo-pagination-block + .demo-pagination-block {
  margin-top: 10px;
}
.demo-pagination-block .demonstration {
  margin-bottom: 16px;
}
</style>