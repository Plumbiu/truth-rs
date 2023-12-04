<script setup lang="ts">
import { ref } from 'vue';
import Graph from './components/Graph.vue'
import Tree from './components/Tree.vue'
let chart = ref('graph')

</script>

<template>
  <button @click="chart = chart === 'graph' ? 'tree' : 'graph'">切换图表</button>
  <component :is=" chart === 'graph' ? Graph : Tree" />
</template>

<style>
#container {
  height: 100vh;
}
</style>