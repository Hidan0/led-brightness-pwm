<script setup lang="ts">
import { computed, ref } from "vue";
// import axios from "axios";
import Led from "@/components/Led.vue";

const brightness = ref(0);
const sliderValue = computed(() => `${brightness.value}%`);

// const BOARD_IP = "192.168.1.17";

const handleChange = async () => {
  try {
    // await axios.put(`http://${BOARD_IP}/brightness?${brightness.value}`);
  } catch (error: any) {
    console.log(error.message);
  }
};
</script>

<template>
  <Led :brightness="brightness" />
  <div class="s-wrapper">
    <input
      class="slider"
      type="range"
      min="0"
      max="100"
      id="led-slider"
      v-model="brightness"
      @change="handleChange"
    />
  </div>
</template>

<style lang="scss">
@use "@/assets/scss/variables";

.led {
  width: 5rem;
  background-color: variables.$red;
}

.s-wrapper {
  display: flex;
  justify-content: center;

  .slider {
    width: 20rem;
    margin: 0 auto;
    height: 3rem;
    outline: none;
    appearance: none;
    border-radius: 3rem;
    border: 0.35rem solid variables.$mantle;

    background: linear-gradient(
      90deg,
      variables.$red v-bind("sliderValue"),
      variables.$mantle v-bind("sliderValue")
    );

    &::-webkit-slider-thumb {
      cursor: pointer;
      appearance: none;
      width: 2rem;
      height: 2rem;
    }
  }
}
</style>
