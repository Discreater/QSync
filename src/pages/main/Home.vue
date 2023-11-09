<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Basic from '~/layouts/Basic.vue';

const { t } = useI18n();
</script>

<template>
  <Basic :header="t('menu.home')" />
</template>
