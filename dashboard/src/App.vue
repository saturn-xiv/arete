<template>
  <router-view/>
</template>

<script>
import { USERS_SIGN_IN } from "@/store";
import { get as getToken } from "@/token";

export default {
  name: "app",
  computed: {
    isSignIn() {
      return this.$store.getters.isSignIn;
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
