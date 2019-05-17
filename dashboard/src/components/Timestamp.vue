<template>
  <span>{{value|moment('llll')}}</span>
</template>

<script>
export default {
  name: "timestamp",
  props: ["value"]
};
</script>
