<script setup lang="ts">
import type { Relation } from '../../../types'

defineProps<{
  data: Relation
}>()
</script>

<template>
  <div v-for="(value, key) in data" :key="key">
    <div v-if="value">
      <div v-if="key === 'dependencies' || key === 'devDependencies'">
        <div class="key" v-if="value">
          {{ key }}
        </div>
        <div
          v-for="(version, name) in value"
          :key="name"
          class="pkg"
          style="padding-left: 16px"
        >
          <div style="line-height: 22px">
            <span>- {{ name }}</span>
            <span class="value">{{ version }}</span>
          </div>
        </div>
      </div>
      <div v-else>
        <span class="key">{{ key }}</span>
        <a
          v-if="key === 'homepage'"
          :href="String(value)"
          target="_blank"
          style="padding-left: 8px"
        >
          {{ value }}
        </a>
        <span v-else class="value">{{ value }}</span>
      </div>
    </div>
  </div>
</template>
