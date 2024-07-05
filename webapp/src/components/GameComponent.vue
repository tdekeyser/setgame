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
