<script setup lang="ts">
import { onMounted, provide, ref } from 'vue'
import type { Relations, PkgInfo } from '@truth-rs/types'
import echarts from '../../plugins/echarts'
import {
  dealGraphNode,
  dealTreeNode,
  formatName,
  initChart,
  request,
} from '../../utils/index'

const base: Relations = await request('relations.json/__root__')
console.log({ base })

const pkgName = ref(base.name)
const pkgInfo = ref<PkgInfo>({ info: base.__root__ })
const isAim = ref(false)
const drawer = ref(false)

provide('drawer', drawer)
provide('pkgName', pkgName)
provide('pkgInfo', pkgInfo)
provide('isAim', isAim)

onMounted(async () => {
  const chartDOM = echarts.init(document.getElementById('chart'))
  await initChart(chartDOM, base)
  chartDOM.on('click', (params: any) => {
    const { data, seriesType, collapsed, treeAncestors } = params
    pkgName.value = data.name
    // pkgInfo.value = getPkgInfo(pkgName.value)
    if (seriesType === 'tree') {
      dealTreeNode(data, collapsed, treeAncestors)
    } else if (!isAim.value) {
      dealGraphNode(data.name)
    }
  })
})
</script>

<template>
  <Header />
  <Drawer />
  <div id="chart" style="height: 100vh" />
</template>
