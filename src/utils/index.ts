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

/**
 * May not work
 */
/* eslint-disable ts/no-this-alias  */
/* eslint-disable ts/no-invalid-this  */
function debounce<T extends (...args: any) => any>(fn: T, wait: number, options?: {
  leading?: boolean
  maxWait?: number
  trailing?: boolean
}): T & { cancel: () => void; flush: () => void; pending: () => boolean } {
  const leading = options?.leading ?? false;
  const maxing = options?.maxWait !== undefined;
  const maxWait = options?.maxWait !== undefined ? Math.max(options?.maxWait ?? 0, wait) : undefined;
  const trailing = options?.trailing ?? true;
  let lastArgs: any[] | undefined;
  let lastThis: any;
  let result: any;
  let timerId: any;
  let lastCallTime: number | undefined;
  let lastInvokeTime = 0;

  // Bypass `requestAnimationFrame` by explicity setting `wait=0`.
  const useRAF = (!wait && wait !== 0 && typeof requestAnimationFrame === 'function');
  function invokeFunc(time: number) {
    const args = lastArgs;
    const thisArg = lastThis;

    lastArgs = lastThis = undefined;
    lastInvokeTime = time;
    result = fn.apply(thisArg, args);
    return result;
  }

  function startTimer(pendingFunc: (time: number) => void, wait: number) {
    if (useRAF) {
      cancelAnimationFrame(timerId);
      return requestAnimationFrame(pendingFunc);
    }
    return setTimeout(pendingFunc, wait);
  }

  function cancelTimer(id: any) {
    if (useRAF)
      return cancelAnimationFrame(id);

    clearTimeout(id);
  }

  function leadingEdge(time: number) {
    // Reset any `maxWait` timer.
    lastInvokeTime = time;
    // Staart the timer for the trailing edge.
    timerId = setTimeout(timerExpired, wait);
    // Invoke the leading edge.
    return leading ? invokeFunc(time) : result;
  }

  function remainingWait(time: number) {
    const timeSinceLastCall = time - lastCallTime!;
    const timeSinceLastInvoke = time - lastInvokeTime;
    const timeWaiting = wait - timeSinceLastCall;

    return maxing ? Math.min(timeWaiting, maxWait! - timeSinceLastInvoke) : timeWaiting;
  }

  function shouldInvoke(time: number) {
    const timeSinceLastCall = time - lastCallTime!; // undefined will be checked in return statement
    const timeSinceLastInvoke = time - lastInvokeTime;

    // Either this is the first call, activity has stopped and we're at the
    // trailing edge, the system time has gone backwards and we're treating
    // it as the trailing edge, or we've hit the `maxWait` limit.
    return (lastCallTime === undefined || (timeSinceLastCall >= wait)
      || (timeSinceLastCall < 0) || (maxing && timeSinceLastInvoke >= maxWait!));
  }

  function timerExpired() {
    const time = Date.now();
    if (shouldInvoke(time))
      return trailingEdge(time);

    // Restart the timer.
    timerId = setTimeout(timerExpired, remainingWait(time));
  }

  function trailingEdge(time: number) {
    timerId = undefined;
    if (trailing && lastArgs)
      return invokeFunc(time);

    lastArgs = lastThis = undefined;
    return result;
  }

  function cancel() {
    if (timerId !== undefined)
      cancelTimer(timerId);

    lastInvokeTime = 0;
    lastArgs = lastCallTime = lastThis = timerId = undefined;
  }

  function flush() {
    return timerId === undefined ? result : trailingEdge(Date.now());
  }

  function pending() {
    return timerId !== undefined;
  }

  function debounced(...args: any[]): ReturnType<T> {
    const time = Date.now();
    const isInvoking = shouldInvoke(time);
    lastArgs = args;
    // @ts-expect-error FIXME I don't know how to fix this
    lastThis = this;
    lastCallTime = time;

    if (isInvoking) {
      if (timerId === undefined)
        return leadingEdge(lastCallTime);

      if (maxing) {
        // Handle invocations in a tight loop.
        timerId = startTimer(timerExpired, wait);
        return invokeFunc(lastCallTime);
      }
    }

    if (timerId === undefined)
      timerId = startTimer(timerExpired, wait);

    return result;
  }

  debounced.cancel = cancel;
  debounced.flush = flush;
  debounced.pending = pending;
  // ts-expect-error FIXME I don't know how to fix this
  return debounced as any;
}

/**
 * May not work
 */
function throttle<T extends (...args: any) => any>(fn: T, wait: number, options?: {
  leading?: boolean
  trailing?: boolean
}) {
  const leading = options?.leading ?? true;
  const trailing = options?.trailing ?? true;
  return debounce(fn, wait, {
    leading,
    maxWait: wait,
    trailing,
  });
}

export const o_o = {
  throttle,
  debounce,
};
