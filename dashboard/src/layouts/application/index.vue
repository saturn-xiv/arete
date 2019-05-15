<template>
  <div>
    <div>application header</div>
    <slot/>
    <div>application footer</div>
  </div>
</template>

<script>
export default {
  name: "application-layout",
  props: {}
};
</script>

<style scoped>
</style>
