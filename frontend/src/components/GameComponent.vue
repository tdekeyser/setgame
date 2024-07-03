<script setup>
import axios from "axios";
import CardComponent from "@/components/CardComponent.vue";
import {onMounted, ref} from "vue";

const game = ref([]);
const triple = ref([]);

const message = ref("");

async function createGame() {
  axios
      .get('/api/game')
      .then((r) => (game.value = r.data));
}

onMounted(() => createGame())

function addToList(card) {
  triple.value.push(card);

  if (triple.value.length === 3) {
    axios
        .post('/api/set', triple.value)
        .then((r) => {
          message.value = r.data;
          triple.value = [];
        })
  }
}
</script>

<template>
  <div class="game">
    <h1>SET game!</h1>

    <h4 class="message">{{ message }}</h4>

    <div class="cards">
      <CardComponent
          v-for="card in game"
          :card="card"
          @click="addToList(card)"
          :class="{'active': triple.includes(card)}"
      />
    </div>
  </div>
</template>

<style scoped>
.game {
  background-color: #ffcc00; /* Bright yellow background */
  color: #000080; /* Dark blue text */
  font-family: 'Comic Sans MS', cursive, sans-serif; /* Comic Sans font for 90s vibe */
  text-align: center;
  padding: 20px;
  border: 5px solid #ff00ff; /* Magenta border */
  margin-left: auto;
  margin-right: auto;
  margin-top: 3%;
  width: 50%;
}

h1 {
  font-size: 3rem;
  text-shadow: 2px 2px #00ff00; /* Green text shadow */
}

.message {
  font-size: 1.5rem;
  margin-bottom: 20px;
  color: #d9534f; /* Bootstrap warning color */
  background-color: #fff3cd; /* Bootstrap warning background */
  border: 1px solid #ffeeba; /* Bootstrap warning border */
  padding: 10px;
  border-radius: 5px;
  box-shadow: 2px 2px #ff00ff; /* Magenta shadow */
}

.cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
  justify-content: center;
}

.active {
  border: 3px solid #00ff00; /* Green border for active cards */
  box-shadow: 5px 5px #000080; /* Magenta shadow */
}
</style>
