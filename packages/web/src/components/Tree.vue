<script setup lang="ts">
import { TreeGraph } from '@antv/g6';
import { onMounted } from 'vue';

onMounted(async () => {
  const container = document.getElementById('tree')!;
  const width = container.scrollWidth;
  const height = container.scrollHeight || 500;
  const graph = new TreeGraph({
    container: 'tree',
    width,
    height,
    modes: {
      default: [
        {
          type: 'collapse-expand',
          onChange: function onChange(item, collapsed) {
            const data = item?.getModel();
            if (!data) {
              return
            }
            data.collapsed = collapsed;
            return true;
          },
        },
        'drag-canvas',
        'zoom-canvas',
      ],
    },
    defaultNode: {
      size: 26,
      anchorPoints: [
        [0, 0.5],
        [1, 0.5],
      ],
    },
    defaultEdge: {
      type: 'cubic-horizontal',
    },
    layout: {
      type: 'compactBox',
      direction: 'LR',
      getId: function getId(d: { id: any; }) {
        return d.id;
      },
      getHeight: function getHeight() {
        return 16;
      },
      getWidth: function getWidth() {
        return 16;
      },
      getVGap: function getVGap() {
        return 10;
      },
      getHGap: function getHGap() {
        return 100;
      },
    },
  });

  graph.node(function (node) {
    return {
      label: node.id,
      labelCfg: {
        offset: 10,
        position: node.children && node.children.length > 0 ? 'left' : 'right',
      },
    };
  });
  const data = await fetch('/tree.json').then(res => res.json())

  graph.data(data);
  graph.render();
  graph.fitView();

  window.onresize = () => {
    if (!graph || graph.get('destroyed')) return;
    if (!container || !container.scrollWidth || !container.scrollHeight) return;
    graph.changeSize(container.scrollWidth, container.scrollHeight);
  }
})

</script>

<template>
  <div id="tree" />
</template>

<style>
#tree {
  height: 100vh;
}
</style>