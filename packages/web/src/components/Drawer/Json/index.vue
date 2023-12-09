<script setup lang="ts">
import Info from './Info.vue'

defineProps<{
  data: any
}>()
</script>

<template>
  <Info :data="data" />
</template>
