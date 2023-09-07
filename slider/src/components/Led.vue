<script setup lang="ts">
import { computed } from "vue";

const props = defineProps({
  brightness: { type: String, required: true },
});

const opacity = computed(() => {
  return parseInt(props.brightness) / 100;
});
</script>

<template>
  <svg
    width="74.605888mm"
    height="99.883018mm"
    viewBox="0 0 74.605888 99.883018"
  >
    <g id="layer1" transform="translate(-67.697054,-55.257014)">
      <g id="g9" transform="translate(-13.28459,22.223324)">
        <g id="g31" transform="translate(-36.812376,-68.792437)">
          <path
            id="led_background"
            style="stroke: #000000; stroke-width: 0; stroke-linejoin: bevel"
            d="m 155.57887,101.82639 c -0.31322,-7.8e-4 -0.48162,0.008 -0.48162,0.008 0,0 -10.79119,-0.34196 -20.08663,8.23568 -5.08676,4.23896 -7.9897,16.50959 -7.9897,16.50959 l 0.0992,31.24719 h -9.32604 v 15.68379 0.11421 l 37.30315,0.11472 v -0.11472 l 37.30263,0.11472 v -15.91272 h -9.35602 l 0.12919,-31.48904 c 0,0 -2.90294,-12.27062 -7.98969,-16.50959 -8.13352,-7.50543 -17.41195,-7.99663 -19.60449,-8.00209 z m -15.96802,71.91345 -0.12764,25.47752 c 0,0 0.16926,0.90308 1.06247,1.53892 0.42389,0.80238 2.14819,0.9524 2.14819,0.9524 0,0 1.05145,0.0455 2.14147,-0.77205 0.92349,-0.66612 1.11259,-1.71152 1.11259,-1.71152 v -25.48527 z m 24.82795,0 -0.12713,25.47752 c 0,0 0.16925,0.90308 1.06247,1.53892 0.42389,0.80238 2.14767,0.9524 2.14767,0.9524 0,0 1.05197,0.0455 2.14199,-0.77205 0.92349,-0.66612 1.11259,-1.71152 1.11259,-1.71152 v -25.48527 z"
          />
        </g>
        <g id="g30" transform="translate(-10.106549,-44.954786)">
          <path
            style="
              fill: none;
              stroke: #000000;
              stroke-width: 0;
              stroke-linejoin: bevel;
            "
            d="m 91.088195,149.67314 v -15.68372 h 74.605885 v 15.9127 z"
            id="path2"
          />
          <path
            style="
              fill: #4d4d4d;
              fill-opacity: 0.25;
              stroke: #000000;
              stroke-width: 0;
              stroke-linejoin: bevel;
            "
            d="m 128.39113,149.78763 v -15.79821 h 37.30295 v 15.9127 z"
            id="path3"
          />
          <path
            style="
              fill: #4d4d4d;
              fill-opacity: 0.1;
              stroke: #000000;
              stroke-width: 0;
              stroke-linejoin: bevel;
            "
            d="m 100.41393,133.98942 h 27.9772 V 77.997159 c 0,0 -10.79092,-0.341933 -20.08637,8.235714 -5.08677,4.238971 -7.98957,16.509587 -7.98957,16.509587 z"
            id="path4"
            sodipodi:nodetypes="cccccc"
          />
          <path
            style="
              fill: #4d4d4d;
              fill-opacity: 0.17;
              stroke: #000000;
              stroke-width: 0;
              stroke-linejoin: bevel;
            "
            d="m 156.33806,133.98942 h -27.94692 l -1e-5,-55.992261 c 0,0 10.79093,-0.584163 20.08639,7.993484 5.08676,4.238971 7.98956,16.509587 7.98956,16.509587 z"
            id="path5"
            sodipodi:nodetypes="cccccc"
          />
          <path
            style="
              fill: #4d4d4d;
              fill-opacity: 0.997753;
              stroke: #000000;
              stroke-width: 0;
              stroke-linejoin: bevel;
            "
            d="m 112.90468,149.90212 h 6.33737 v 25.48527 c 0,0 -0.18918,1.04535 -1.11267,1.71147 -1.09002,0.81752 -2.14166,0.7721 -2.14166,0.7721 0,0 -1.7241,-0.15021 -2.14799,-0.95259 -0.89321,-0.63584 -1.06242,-1.53853 -1.06242,-1.53853 z"
            id="path6"
            sodipodi:nodetypes="cccccccc"
          />
          <path
            style="
              fill: #4d4d4d;
              fill-opacity: 0.997753;
              stroke: #000000;
              stroke-width: 0;
              stroke-linejoin: bevel;
            "
            d="m 137.73293,149.90212 h 6.33738 v 25.48527 c 0,0 -0.18918,1.04535 -1.11267,1.71147 -1.09002,0.81752 -2.14166,0.7721 -2.14166,0.7721 0,0 -1.7241,-0.15021 -2.14799,-0.95259 -0.89322,-0.63584 -1.06243,-1.53853 -1.06243,-1.53853 z"
            id="path7"
            sodipodi:nodetypes="cccccccc"
          />
          <path
            style="
              fill: #4d4d4d;
              fill-opacity: 0.18;
              stroke: #000000;
              stroke-width: 0;
              stroke-linejoin: bevel;
            "
            d="m 91.088195,149.78763 v -15.79821 h 37.302945 v 15.9127 z"
            id="path8"
          />
        </g>
      </g>
    </g>
  </svg>
</template>

<style lang="scss">
@use "@/assets/scss/variables";

#led_background {
  --alpha: v-bind("opacity");
  fill: rgba(variables.$red, var(--alpha));
}
</style>
