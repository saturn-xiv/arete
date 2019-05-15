<template>
  <application-layout
    v-bind:alert="alert"
    v-bind:onSubmit="submit"
    v-bind:title="this.$t('nut.users.sign-in.title')"
  >
    <v-form>
      <v-text-field
        prepend-icon="person"
        name="login"
        v-model="login"
        :error-messages="errors.collect('login')"
        v-validate="'required'"
        :label="this.$t('nut.users.sign-in.login')"
        type="text"
      />
      <v-text-field
        id="password"
        prepend-icon="lock"
        v-model="password"
        :error-messages="errors.collect('password')"
        v-validate="'required'"
        name="password"
        :label="this.$t('form.labels.password')"
        type="password"
      />
    </v-form>
  </application-layout>
</template>

<script>
import client from "@/request";
import { USERS_SIGN_IN } from "@/store";

export default {
  name: "users-sign-in",
  data() {
    return { login: null, password: null, alert: {} };
  },
  methods: {
    async submit(e) {
      e.preventDefault();
      this.alert = {};
      const isValid = await this.$validator.validate();
      if (isValid) {
        client
          .post("/users/sign-in", {
            login: this.login,
            password: this.password
          })
          .then(res => {
            this.$store.commit(USERS_SIGN_IN, res.data);
            this.$router.push({ name: "home" });
          })
          .catch(error => {
            this.alert = { ok: false, message: error.response.data };
          });
      }
    }
  }
};
</script>
