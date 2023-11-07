import { ref } from 'vue';

const loading = ref(true);

export function useLoading() {
  return loading;
}
