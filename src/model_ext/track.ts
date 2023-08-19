export type TrackId = number;

export const TrackExt = {
  durationInSecs: (track: Track): number => {
    return Math.floor(track.duration / 1000);
  },
};
