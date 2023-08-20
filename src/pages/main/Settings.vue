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
import { useQSyncStore } from '~/store';
import QList from '~/components/QList.vue';

const { t } = useI18n();
const store = useQSyncStore();

async function handleAddMusicFolder() {
  const selected = await open({
    directory: true,
  }) as string;
  // store.addMusicFolder(selected);
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
        <QList :items="store.musicFolders" :key-map="(t) => t.path">
          <template #item="{ item }">
            <div>{{ item.path }}</div>
          </template>
          <template #empty>
            {{ t('settings.lib-no-folder') }}
          </template>
        </QList>
      </template>
    </LongButton>
  </Basic>
</template>
