<template>
  <div/>
</template>

<script>
export default {
  name: "home",
  computed: {
    isSignIn() {
      return this.$store.getters.isSignIn;
    }
  },
  created() {
    this.$router.push({
      name: this.isSignIn ? "users.logs" : "users.sign-in"
    });
  }
};
</script>
