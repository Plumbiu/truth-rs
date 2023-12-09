<script setup lang="ts">
import { type Ref, inject, ref } from 'vue'
import { ArrowDown } from '@element-plus/icons-vue'
import { getPkgInfo } from '../../utils/index'
import type { Relation, ShowType } from '../../types'

const pkgName = inject<Ref<string>>('pkgName')!
const pkgInfo = inject<Ref<Relation>>('pkgInfo')!
const drawer = inject<Ref<boolean>>('drawer')
const type = ref<ShowType>('info')

function handlePkgInfo() {
  pkgInfo.value = getPkgInfo(pkgName.value)
}
</script>

<template>
  <Transition>
    <div v-if="drawer" class="drawer">
      <ElScrollbar>
        <div class="drawer_header">
          <div class="pkgName">
            <ElScrollbar>
              {{ pkgName }}
            </ElScrollbar>
          </div>
          <ElDropdown trigger="click" @command="handlePkgInfo">
            <ElButton>
              依赖信息
              <ElIcon title="查看依赖信息" class="el-icon--right">
                <ArrowDown />
              </ElIcon>
            </ElButton>
            <template #dropdown>
              <ElButton
                tag="a"
                target="_blank"
                :href="`https://npmjs.com/package/${pkgName}`"
              >
                NPM
              </ElButton>
            </template>
          </ElDropdown>
        </div>
        <ElScrollbar
          style="
            font-size: 14px;
            color: var(--el-text-color-primary);
            line-height: 26px;
          "
        >
          <div v-if="pkgInfo">
            <Json :data="pkgInfo" :type="type" />
          </div>
          <div v-else class="value">未找到依赖信息</div>
        </ElScrollbar>
      </ElScrollbar>
    </div>
  </Transition>
</template>

<style scoped>
.drawer {
  position: absolute;
  z-index: 998;
  top: 50px;
  height: 100%;
  padding-bottom: 80px;
  width: 300px;
  translate: 0px;
  padding: 16px;
  background-color: var(--el-bg-color);
  box-shadow: var(--el-box-shadow-light);
}

.drawer_header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
  padding-right: 12px;
}

.v-enter-active,
.v-leave-active {
  transition: translate 0.15s ease-in-out;
}

.v-enter-from,
.v-leave-to {
  translate: -300px;
}

.pkgName {
  font-weight: 700;
  font-size: 20px;
  overflow: hidden;
  white-space: nowrap;
  color: var(--el-text-color-primary);
}
</style>

<style>
.el-button + .el-button {
  margin-left: 0;
}
</style>
