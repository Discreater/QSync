<script setup lang="ts">
import Navigator from '~/components/Navigator.vue';
</script>

<template>
  <div class="h-full flex-1 flex flex-col sm:block sm:relative md:flex md:flex-row md:static overflow-hidden">
    <Navigator class="shrink-0 static sm:absolute md:static z-10 h-auto sm:h-full" />
    <div class="w-full h-full sm:ml-12 sm:w-[calc(100%-3rem)] md:ml-0">
      <RouterView />
    </div>
  </div>
</template>
