<template>
  <div class="common-layout">
    <el-container>
      <el-header><BaseHeader></BaseHeader></el-header>
      <el-container>
        <el-aside width="150px"><BaseSide></BaseSide></el-aside>
        <el-container>
          <el-main>
            <router-view >
            </router-view>
          </el-main>
        </el-container>
      </el-container>
    </el-container>
  </div>
</template>

<script lang="ts" setup>
</script>

<style scoped>
.el-col {
  border-radius: 4px;
}

.grid-content {
  border-radius: 4px;
  min-height: 36px;
}
</style>