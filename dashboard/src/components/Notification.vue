<template>
  <v-snackbar :color="color" v-model="show">{{alert.message}} {{new Date()|moment('llll')}}</v-snackbar>
</template>

<script>
export default {
  name: "notification-bar",
  computed: {
    color() {
      return this.alert.ok ? "success" : "error";
    },
    show: {
      get() {
        return this.alert.message != null;
      },
      set(v) {}
    }
  },
  props: {
    alert: Object
  }
};
</script>
