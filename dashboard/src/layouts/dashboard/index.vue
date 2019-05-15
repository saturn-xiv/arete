<template>
  <div>
    <div>dashboard header</div>
    <slot/>
    <div>dashboard footer</div>
  </div>
</template>

<script>
export default {
  name: "dashboard-layout",
  props: {}
};
</script>

<style scoped>
</style>
