const AutoImport = require('unplugin-auto-import/webpack')
const Components = require('unplugin-vue-components/webpack')
const { ElementPlusResolver } = require('unplugin-vue-components/resolvers')
const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  transpileDependencies: true,
  lintOnSave:false,
  outputDir:"../static",
  css:{
    loaderOptions:{
      scss:{
        additionalData:`@use "./src/styles/element/index.scss" as *;`
      }
    }
  },
  devServer:{
    host:'0.0.0.0',
    proxy:{
      '/ui':{
        target:"http://127.0.0.1:4000"
      }
    }
  },
  configureWebpack:{
    plugins:[
      AutoImport({
        resolvers: [
          ElementPlusResolver(),
        ],
        dts: 'src/auto-components.d.ts',
      }),
      Components({
        extensions: ['vue', 'md'],
        include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
        resolvers: [
          ElementPlusResolver({
            importStyle: 'sass',
          }),
        ],
        dts: 'src/components.d.ts',
      }),
    ]
  }
})
