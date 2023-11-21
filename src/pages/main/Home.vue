<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Basic from '~/layouts/Basic.vue';

const { t } = useI18n();
</script>

<template>
  <Basic :header="t('menu.home')">
    - English translation is not complete yet. Nothing here.
    <br>
    - 主页是空的，因为我懒得做。你需要以 `Kerse` 账号登陆，登录之后可能还要重启应用 :(。
    <br>
    - 为了有歌听，然后你还要在 `源` 页面中添加本地音乐文件夹。
  </Basic>
</template>
