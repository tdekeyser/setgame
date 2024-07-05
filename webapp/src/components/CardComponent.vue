<script setup>
defineProps({
  card: {
    type: Array,
    required: true,
  }
});

function getImageSrc(card) {
  return './set/' + card[2] + card[1] + card[3] + '.png';
}

function getAmount(card) {
  let amountMap = {
    'One': 1,
    'Two': 2,
    'Three': 3,
  }
  return [...Array(amountMap[card[0]]).keys()]
}
</script>

<template>
  <div class="card">
    <img class="card-img" v-for="_ in getAmount(card)" :src="getImageSrc(card)" :alt="card" />
  </div>
</template>
