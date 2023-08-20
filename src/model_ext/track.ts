export type TrackId = number;

export const TrackExt = {
  durationInSecs: (duration: number): number => {
    return duration / 1000;
  },
};
