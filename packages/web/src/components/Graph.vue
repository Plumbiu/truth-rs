<script setup lang="ts">
import { IG6GraphEvent, Graph } from '@antv/g6';
import { onMounted } from 'vue';

onMounted(async () => {
  const container = document.getElementById('container')!;
  const width = container.clientWidth;
  const height = container.clientHeight || 500;
  const graph = new Graph({
    container: 'container',
    width,
    height,
    minZoom: 0.25,
    maxZoom: 4,
    layout: {
      type: 'gForce',
      animate: true,
      linkDistance: 50,
      nodeStrength: 1200,
    },
    modes: {
      default: ['drag-canvas', 'zoom-canvas', 'drag-node', {
        type: 'tooltip', // 提示框
        formatText(model) {
          // 提示框文本内容
          const text = '<b>label:</b> ' + model.label + '<br/> <b>class:</b> ' + model.class;
          return text;
        },
      }]
    },
  });
  const data = await fetch('/graph.json').then(res => res.json())
  graph.data({
    nodes: data.nodes,
    edges: data.links,
  });
  graph.render();

  graph.on('node:dragstart', (e) => {
    graph.layout();
    refreshDragedNodePosition(e);
  });
  graph.on('node:drag', (e) => {
    const forceLayout = graph.get('layoutController').layoutMethods[0];
    forceLayout.execute();
    refreshDragedNodePosition(e);
  });
  graph.on('node:dragend', (e) => {
    if (!e?.item) {
      return
    }
    e.item.get('model').fx = null;
    e.item.get('model').fy = null;
  });

  window.onresize = () => {
    if (!graph || graph.get('destroyed')) return;
    if (!container || !container.scrollWidth || !container.scrollHeight) return;
    graph.changeSize(container.scrollWidth, container.scrollHeight);
  };

  function refreshDragedNodePosition(e: IG6GraphEvent) {
    if (!e?.item) {
      return
    }
    const model = e.item.get('model');
    model.fx = e.x;
    model.fy = e.y;
  }

})

</script>

<template>
  <div id="container" />
</template>

<style>
#container {
  height: 100vh;
}
</style>