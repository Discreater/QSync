<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Basic from '~/layouts/Basic.vue';
import H1 from '~/components/typo/H1.vue';
import H2 from '~/components/typo/H2.vue';
import LongButton from '~/components/LongButton.vue';
import Button from '~/components/Button.vue';

import IconFolder from '~icons/fluent/folder-24-regular';
import IconFolderAdd from '~icons/fluent/folder-add-24-regular';

import { open } from '~/platforms/dialog';
import { useQSyncStore } from '~/store';
import List from '~/components/List.vue';

const { t } = useI18n();
const store = useQSyncStore();

async function handleAddMusicFolder() {
  const selected = await open({
    directory: true,
  }) as string;
  store.addMusicFolder(selected);
}
</script>

<template>
  <Basic>
    <H1>{{ t('menu.settings') }}</H1>
    <H2>{{ t("settings.library") }}</H2>
    <LongButton :icon="IconFolder" :droppable="true" :text="t('settings.music-lib-position')">
      <template #extra>
        <Button :icon="IconFolderAdd" :text="t('settings.add-folder')" @click.stop="handleAddMusicFolder()" />
      </template>
      <template #drop>
        <List :items="store.musicFolders">
          <template #item="{ item }">
            {{ item }}
          </template>
          <template #empty>
            {{ t('settings.lib-no-folder') }}
          </template>
        </List>
      </template>
    </LongButton>
  </Basic>
</template>
