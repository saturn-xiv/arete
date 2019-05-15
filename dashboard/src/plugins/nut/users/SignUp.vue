<template>
  <application-layout v-bind:onSubmit="submit" v-bind:title="this.$t('nut.users.sign-up.title')">
    <v-form>
      <v-text-field
        prepend-icon="person"
        name="nickname"
        v-model="nickname"
        :label="this.$t('form.labels.nickname')"
        type="text"
      />
      <v-text-field
        id="password"
        prepend-icon="lock"
        v-model="password"
        name="password"
        :label="this.$t('form.labels.password')"
        type="password"
      />
    </v-form>
  </application-layout>
</template>

<script>
export default {
  name: "users-sign-up",
  data() {
    return { nickname: null, password: null };
  },
  methods: {
    submit(e) {
      e.preventDefault();
      console.log("submit" + this.password);
    }
  }
};
</script>
