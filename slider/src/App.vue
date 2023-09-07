<script setup lang="ts">
import Slider from "@/components/Slider.vue";
</script>

<template>
  <div class="container">
    <h1 class="title">Led brightness</h1>
    <Slider />
  </div>
</template>

<style lang="scss">
@import "@/assets/scss/main";

.title {
  text-align: center;
  padding: 1rem 0;
}
</style>
