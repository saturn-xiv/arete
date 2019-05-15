<template>
  <application-layout
    v-bind:alert="alert"
    v-bind:onSubmit="submit"
    v-bind:title="this.$t('nut.install.title')"
  >
    <v-form>
      <v-text-field
        prepend-icon="person"
        name="realName"
        v-model="realName"
        :error-messages="errors.collect('realName')"
        v-validate="'required'"
        :label="this.$t('form.labels.real-name')"
        type="text"
      />
      <v-text-field
        prepend-icon="email"
        name="email"
        :error-messages="errors.collect('email')"
        v-model="email"
        v-validate="'required|email'"
        :label="this.$t('form.labels.email')"
        type="email"
      />
      <v-text-field
        id="password"
        prepend-icon="lock"
        :error-messages="errors.collect('password')"
        v-model="password"
        ref="password"
        name="password"
        v-validate="'required|min:6|max:32'"
        :label="this.$t('form.labels.password')"
        type="password"
      />
      <v-text-field
        id="passwordConfirmation"
        :error-messages="errors.collect('passwordConfirmation')"
        prepend-icon="lock"
        v-validate="'required|confirmed:password'"
        v-model="passwordConfirmation"
        name="passwordConfirmation"
        :label="this.$t('form.labels.password-confirmation')"
        type="password"
      />
    </v-form>
  </application-layout>
</template>

<script>
import client from "@/request";

export default {
  name: "install",
  data() {
    return {
      email: null,
      realName: null,
      password: null,
      passwordConfirmation: null,
      alert: {}
    };
  },
  methods: {
    async submit(e) {
      e.preventDefault();
      this.alert = {};
      const isValid = await this.$validator.validate();
      if (isValid) {
        client
          .post("/install", {
            email: this.email,
            realName: this.realName,
            password: this.password
          })
          .catch(error => {
            this.alert = { ok: false, message: error.response.data };
          });
      }
    }
  }
};
</script>

