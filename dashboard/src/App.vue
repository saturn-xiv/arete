<template>
  <div>
    <router-view/>
    <v-snackbar
      :color="notification.color"
      v-model="show"
    >{{notification.message}} {{new Date()|moment('llll')}}</v-snackbar>
  </div>
</template>

<script>
import { USERS_SIGN_IN } from "@/store";
import { get as getToken } from "@/token";

export default {
  name: "app",
  computed: {
    isSignIn() {
      return this.$store.getters.isSignIn;
    },
    notification() {
      return this.$store.getters.notification;
    },
    show: {
      get() {
        return this.$store.getters.notification.message != null;
      },
      set() {}
    }
  },
  created() {
    const token = getToken();
    if (token) {
      if (!this.isSignIn) {
        this.$store.commit(USERS_SIGN_IN, token);
      }
    }
  }
};
</script>

<style scoped>
</style>
