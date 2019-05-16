<template>
  <application-layout v-bind:onSubmit="submit" v-bind:title="this.$t('nut.users.sign-in.title')">
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
import { post as httpPost } from "@/request";
import { set as setToken } from "@/token";
import { USERS_SIGN_IN, NOTIFICATION_ERROR } from "@/store";

export default {
  name: "users-sign-in",
  data() {
    return { login: null, password: null };
  },
  methods: {
    async submit(e) {
      e.preventDefault();

      const isValid = await this.$validator.validate();
      if (isValid) {
        httpPost("/users/sign-in", {
          login: this.login,
          password: this.password
        })
          .then(res => {
            setToken(res);
            this.$store.commit(USERS_SIGN_IN, res);
            this.$router.push({ name: "home" });
          })
          .catch(err => {
            this.$store.commit(NOTIFICATION_ERROR, err);
          });
      }
    }
  }
};
</script>
