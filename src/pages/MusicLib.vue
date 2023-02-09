<template>
  <div>
    Music lib
    <p>
      The Caterpillar and Alice looked at each other for some time in silence: at
      last the Caterpillar took the hookah out of its mouth, and addressed her in a
      languid, sleepy voice.
    </p>
  </div>
</template>

<style scoped>
p {
  animation-duration: 3s;
  animation-name: slidein;
}

@keyframes slidein {
  from {
    margin-left: 100%;
    width: 300%;
  }

  to {
    margin-left: 0%;
    width: 100%;
  }
}
</style>
