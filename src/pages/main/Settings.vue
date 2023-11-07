<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Basic from '~/layouts/Basic.vue';
import H1 from '~/components/typo/H1.vue';
import H2 from '~/components/typo/H2.vue';
import LongButton from '~/components/LongButton.vue';
import QButton from '~/components/QButton.vue';

import IconFolder from '~icons/fluent/folder-24-regular';
import IconFolderAdd from '~icons/fluent/folder-add-24-regular';

import { open } from '~/platforms/dialog';
import { useMusyncStore } from '~/store';
import QList from '~/components/QList.vue';
import { ApiClient } from '~/api/client';

const { t } = useI18n();
const store = useMusyncStore();

async function handleAddMusicFolder() {
  const _selected = await open({
    directory: true,
  }) as string;
  // store.addMusicFolder(selected);
}

async function rebuildIndex() {
  ApiClient.get().grpcClient.RebuildIndex({});
}
</script>

<template>
  <Basic>
    <H1>{{ t('menu.settings') }}</H1>
    <H2>{{ t("settings.user") }}</H2>
    <LongButton :icon="IconFolder" :droppable="true" :text="t('settings.music-lib-position')">
      <template #extra>
        <QButton :icon="IconFolderAdd" :text="t('settings.add-folder')" @click.stop="handleAddMusicFolder()" />
      </template>
      <template #drop>
        <QList :items="store.musicFolders" :key-map="({ path }) => path">
          <template #item="{ item }">
            <div>{{ item.path }}</div>
          </template>
          <template #empty>
            {{ t('settings.lib-no-folder') }}
          </template>
        </QList>
      </template>
    </LongButton>
    <QButton :text="t('settings.reindex')" @click="rebuildIndex" />
  </Basic>
</template>
