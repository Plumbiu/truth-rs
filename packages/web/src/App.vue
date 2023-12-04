<script setup lang="ts">
import G6, { IG6GraphEvent } from '@antv/g6';
import { onMounted } from 'vue';


onMounted(async () => {

  const container = document.getElementById('container')!;
  const width = container.clientWidth;
  const height = container.clientHeight || 500;
  const graph = new G6.Graph({
    container: 'container',
    width,
    height,
    layout: {
      type: 'force',
    },
    defaultNode: {
      size: 15,
    },
  });

  fetch('/graph.json')
    .then((res) => res.json())
    .then((data) => {
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

      if (typeof window !== 'undefined')
        window.onresize = () => {
          if (!graph || graph.get('destroyed')) return;
          if (!container || !container.scrollWidth || !container.scrollHeight) return;
          graph.changeSize(container.scrollWidth, container.scrollHeight);
        };
    });

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