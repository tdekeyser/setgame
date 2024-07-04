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

<style scoped>
.card {
  display: flex;
  background-color: #fabedc; /* Hot pink background */
  border: 3px dashed #00ffff; /* Cyan dashed border */
  border-radius: 10px;
  padding: 10px;
  margin: 10px;
  text-align: center;
  box-shadow: 5px 5px #ff00ff; /* Magenta shadow */
}

.card-img {
  margin-left: auto;
  margin-right: auto;
}
</style>
