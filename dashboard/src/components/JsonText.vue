<template>
  <pre>{{JSON.stringify(value, null, 2)}}</pre>
</template>

<script>
export default {
  name: "json-text",
  props: ["value"]
};
</script>
