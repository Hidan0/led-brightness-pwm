<script setup lang="ts">
import { computed, ref } from "vue";
import axios from "axios";
import Led from "@/components/Led.vue";

const brightness = ref(0);
const sliderValue = computed(() => `${brightness.value}%`);

const BOARD_IP = "192.168.1.17";

const instance = axios.create({ baseURL: `http://${BOARD_IP}` });

const handleChange = async () => {
  try {
    await instance.put(`/brightness?${brightness.value}`);
  } catch (error: any) {
    console.log(error.message);
  }
};

const fetchCurrentBrightness = async () => {
  try {
    const res = await instance.get("/brightness");
    brightness.value = Math.round((res.data.current / res.data.max) * 100);
  } catch (error: any) {
    console.log("Can not get current brightness due to: " + error.message);
  }
};

fetchCurrentBrightness();
</script>

<template>
  <div class="s-wrapper">
    <Led class="led" :brightness="brightness.toString()" />
    <span class="slider-value">{{ sliderValue }}</span>
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

.s-wrapper {
  display: flex;
  justify-content: center;
  flex-flow: column;

  .led {
    width: 5rem;
    height: 6.25rem;
    margin: 1rem auto;
  }

  .slider-value {
    font-weight: bold;
    font-size: 1.25rem;
    margin: 0 auto;
  }

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
