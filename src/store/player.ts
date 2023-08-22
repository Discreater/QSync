import { defineStore } from 'pinia';
import { ApiClient } from '~/api/client';
import type { UpdatePlayerEvent, UpdatePlayerRequest } from '~/generated/protos/musync';
import { logger } from '~/utils/logger';

export interface PlayerStore {
  playing: boolean
  position: number
  // When not playing, it represents the progress directly. In milliseconds.
  startAt: number
  repeat: boolean
}

function evilProgress(playing: boolean, startAt: number): number {
  const now = Date.now();
  return playing ? now - startAt : startAt;
}

function fromEvilProgress(playing: boolean, progress: number): number {
  const now = Date.now();
  return playing ? now - progress : progress;
}

export const usePlayerStore = defineStore('playQueueStatus', {
  state: () => ({
    playing: false,
    position: 0,
    startAt: 0,
    repeat: false,
  } as PlayerStore),
  actions: {
    progress(): number {
      return evilProgress(this.playing, this.startAt);
    },
    updateFromRemote(event: UpdatePlayerEvent) {
      const startAt = fromEvilProgress(event.playing, event.progress);

      this.$patch({
        playing: event.playing,
        position: event.position,
        startAt,
      });
    },
    updateToRemote(request: UpdatePlayerRequest) {
      const playing = request.playing ?? this.playing;

      const progress = request.progress ?? this.progress();
      const startAt = fromEvilProgress(playing, progress);

      const position = request.position ?? this.position;
      this.$patch({
        playing,
        position,
        startAt,
      });
      ApiClient.ws()?.sendMsg({ UpdatePlayer: request });
    },
    play(current: number) {
      this.updateToRemote({
        manul: true,
        playing: true,
        position: current,
        progress: 0,
      });
    },
    togglePlay() {
      logger.trace('toggle play');
      this.updateToRemote({
        manul: true,
        playing: !this.playing,
      });
    },
    pauseTrack() {
      logger.trace('pause track');
      this.updateToRemote({
        manul: true,
        playing: false,
      });
    },
    resumeTrack() {
      logger.trace('resume track');
      this.updateToRemote({
        manul: true,
        playing: true,
      });
    },
    /** @param progress should be in milliseconds */
    updateProgress(progress: number, manul: boolean) {
      this.updateToRemote({
        manul,
        progress,
      });
    },
    updatePosition(position: number, manul: boolean) {
      this.updateToRemote({
        manul,
        position,
        progress: 0,
      });
    },
  },
  persist: true,
});
