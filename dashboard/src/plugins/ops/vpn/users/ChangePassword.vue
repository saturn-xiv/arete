<template>
  <application-layout v-bind:onSubmit="submit" v-bind:title="title">
    <v-form>
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
        prepend-icon="lock"
        name="currentPassword"
        :error-messages="errors.collect('currentPassword')"
        v-model="currentPassword"
        v-validate="'required'"
        :label="this.$t('form.labels.current-password')"
        type="password"
      />
      <v-text-field
        id="newPassword"
        prepend-icon="lock"
        :error-messages="errors.collect('newPassword')"
        v-model="newPassword"
        ref="newPassword"
        name="newPassword"
        v-validate="'required|min:6|max:32'"
        :label="this.$t('form.labels.new-password')"
        type="password"
      />
      <v-text-field
        id="passwordConfirmation"
        :error-messages="errors.collect('passwordConfirmation')"
        prepend-icon="lock"
        v-validate="'required|confirmed:newPassword'"
        v-model="passwordConfirmation"
        name="passwordConfirmation"
        :label="this.$t('form.labels.password-confirmation')"
        type="password"
      />
    </v-form>
  </application-layout>
</template>

<script>
import { post as httpPost } from "@/request";
import { NOTIFICATION_ERROR, NOTIFICATION_SUCCESS } from "@/store";

export default {
  name: "ops-vpn-users-change-password",
  computed: {
    title() {
      return this.$i18n.t("ops.vpn.users.change-password.title");
    }
  },
  data() {
    return {
      currentPassword: null,
      newPassword: null,
      passwordConfirmation: null,
      email: null
    };
  },
  methods: {
    async submit(e) {
      e.preventDefault();

      const isValid = await this.$validator.validate();
      if (isValid) {
        httpPost("/ops/vpn/users/change-password", {
          email: this.email,
          currentPassword: this.currentPassword,
          newPassword: this.newPassword
        })
          .then(() => {
            this.$store.commit(
              NOTIFICATION_SUCCESS,
              this.$i18n.t("flashes.success")
            );
          })
          .catch(err => {
            this.$store.commit(NOTIFICATION_ERROR, err);
          });
      }
    }
  }
};
</script>
