<script setup lang="ts">
import { ref } from 'vue'
let chart = ref('/')
</script>

<template>
  <RouterLink :to="chart" @click="chart = chart === '/' ? 'tree' : '/'">
    切换图表
  </RouterLink>
  <RouterView />
</template>

<style>
#container {
  height: 100vh;
}
</style>
