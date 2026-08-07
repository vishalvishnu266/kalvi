<script setup>
import { ref } from 'vue'

import UiDatepicker from "./UiDatepicker.vue";

const count = ref(0)
</script>

<template>
    <button type="button" class="counter" @click="count++">
      Count is {{ count }}
    </button>
    Count is {{ count }}
<ui-datepicker></ui-datepicker>


</template>
