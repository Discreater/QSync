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
      array[randomIndex], array[currentIndex]];
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
