<template>
  <div>
    <el-row justify="space-between" gutter="40">
      <el-col :span="13">
        <el-table :data="libraryStore.librarys" style="width: 100%" max-height="100%"
        stripe border
                  :default-sort="{prop:size,order:'descending'}"
                  @row-click="ToDetails"
        >
          <el-table-column fixed prop="name" :label="$t('message.LibraryName')"  />
          <el-table-column prop="size" :label="$t('message.LibrarySize')" />
          <el-table-column prop="images_count" :label="$t('message.RepCount')"  />
          <el-table-column prop="tags_count" :label="$t('message.TagCount')"  />
          <el-table-column fixed="right" :label="$t('message.Operation')" width="120">
            <template #default="scope">
              <el-button
                  link
                  type="primary"
                  size="small"
                  @click.stop="deleteRow(scope.row)"
              >
                {{$t('message.remove')}}
              </el-button>
            </template>
          </el-table-column>
        </el-table>
        <div class="demo-pagination-block" style="margin-top: 20px">
<!--          <div class="demonstration">Change page size</div>-->
          <el-pagination
              v-model:currentPage="currentPage"
              v-model:page-size="pageSize"
              :page-sizes="[5, 10, 15, 20]"
              :disabled="disabled"
              :background="background"
              layout="sizes, prev, pager, next"
              :total="libraryStore.library_total"
              @size-change="handleSizeChange"
              @current-change="handleCurrentChange"
          />
        </div>
      </el-col>
      <el-col :span="11">
        <el-card shadow="hover" style="height: 600px" >
          <div id="test" style="width: 100%;height: 600px;"></div>
        </el-card>
      </el-col>
    </el-row>
  </div>
  <DelLibrary></DelLibrary>
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
import {useRoute} from "vue-router/dist/vue-router";
import {useRouter} from "vue-router";
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
  name: "LibraryList",
  setup() {
    let libraryStore = useLibraryStore();
    // const route = useRoute();
    const router = useRouter();
    let myEcharts=""
    const currentPage = ref(1)
    const pageSize = ref(10)
    const background = ref(true)
    const disabled = ref(false)

    const handleSizeChange = (val) => {
      libraryStore.GetLibrarys(val,0).then(()=>{
        let generatOptions = generat_options(libraryStore.getName,libraryStore.getCharData);
        myEcharts.setOption(generatOptions);
      })
    }
    function ToDetails(row,column){
        console.log("click row",row.name)
        router.push({name:"Image",query:{library:row.name}})
        console.log("click column",column)
    }
    const handleCurrentChange = (val) => {
      libraryStore.GetLibrarys(pageSize.value,(currentPage.value-1)*pageSize.value).then(()=>{
        let generatOptions = generat_options(libraryStore.getName,libraryStore.getCharData);
        myEcharts.setOption(generatOptions);
      })

    }
    function deleteRow(id) {
      emitter.emit("delete_library", id)
    }
    emitter.on("update_charts",()=>{
      libraryStore.GetLibraryTotal().then(()=>{
        let generatOptions = generat_options(libraryStore.getName,libraryStore.getCharData);
        myEcharts.setOption(generatOptions);
      })
    })
    onMounted(() => {
      console.log("mount start")
      libraryStore.GetLibraryTotal()
      libraryStore.GetLibrarys(pageSize.value,0).then(() => {
        myEcharts = echarts.init(document.getElementById("test"))
        window.addEventListener("resize", () => {
          myEcharts.resize()
        })
        let generatOptions = generat_options(libraryStore.getName,libraryStore.getCharData);
        // console.log(generatOptions)
        myEcharts.setOption(generatOptions);
      })
    })

      return {
        libraryStore, deleteRow,myEcharts,disabled,currentPage,background,pageSize,handleCurrentChange,handleSizeChange,ToDetails
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