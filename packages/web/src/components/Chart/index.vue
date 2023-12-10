<script setup lang="ts">
import { onMounted, provide, ref } from 'vue'
import type { Relation } from '../../types'
import echarts from '../../plugins/echarts'
import {
  dealGraphNode,
  dealTreeNode,
  getPkgInfo,
  initChart,
  request,
} from '../../utils/index'

const relation: Relation = await request('relations.json/query?name=__root__')
const pkgName = ref(relation.name ?? '__root__')
const pkgInfo = ref<Relation>(relation)
const isAim = ref(false)
const drawer = ref(false)

provide('drawer', drawer)
provide('pkgName', pkgName)
provide('pkgInfo', pkgInfo)
provide('isAim', isAim)

onMounted(async () => {
  const chartDOM = echarts.init(document.getElementById('chart'))
  await initChart(chartDOM)
  // @ts-ignore
  chartDOM.on('click', async (params: any) => {
    const { data, seriesType, collapsed, treeAncestors } = params
    pkgName.value = data.name
    pkgInfo.value = await getPkgInfo(pkgName.value)
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
