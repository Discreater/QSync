export * from './dark';

/** Will modify the array */
export function shuffle<T>(array: T[]): T[] {
  let currentIndex = array.length;
  let randomIndex;

  // While there remain elements to shuffle.
  while (currentIndex !== 0) {
    // Pick a remaining element.
    randomIndex = Math.floor(Math.random() * currentIndex);
    currentIndex--;

    // And swap it with the current element.
    [array[currentIndex], array[randomIndex]] = [
      array[randomIndex],
      array[currentIndex],
    ];
  }

  return array;
}

/** Make sure return positive */
export function mod(n: number, m: number) {
  return ((n % m) + m) % m;
}

export function pad(num: number | string, length: number): string {
  const str = typeof num === 'number' ? `${num}` : num;
  return str.padStart(length, '0');
}

/** @param time in seconds */
export function formatTime(time: number, formatter: 'mm:ss' | 'hh:mm:ss' = 'mm:ss') {
  if (formatter === 'hh:mm:ss') {
    const hour = Math.floor(time / 3600);
    const min = pad(Math.floor(time / 60 % 60), 2);
    const sec = pad(Math.floor(time % 60), 2);
    return `${hour}:${min}:${sec}`;
  } else {
    const minutes = pad(Math.floor(time / 60), 2);
    const seconds = pad(Math.floor(time % 60), 2);
    return `${minutes}:${seconds}`;
  }
}
