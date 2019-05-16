<template>
  <dashboard-layout v-bind:title="title">
    <v-flex md6>
      <v-card>
        <v-card-title primary-title>
          <h3 class="headline mb-0">{{title}}</h3>
        </v-card-title>
        <v-form>
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
        <v-card-actions>
          <v-spacer/>
          <v-btn v-on:click="submit" color="primary">{{$t('buttons.submit')}}</v-btn>
        </v-card-actions>
      </v-card>
    </v-flex>
    <notification-bar :alert="alert"/>
  </dashboard-layout>
</template>

<script>
import { post as httpPost } from "@/request";

export default {
  name: "users-change-password",
  computed: {
    title() {
      return this.$i18n.t("nut.users.change-password.title");
    }
  },
  data() {
    return {
      currentPassword: null,
      newPassword: null,
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
        httpPost("/users/change-password", {
          currentPassword: this.currentPassword,
          newPassword: this.newPassword
        })
          .then(() => {
            this.alert = { ok: true, message: this.$i18n.t("flashes.success") };
          })
          .catch(err => {
            this.alert = { ok: false, message: err };
          });
      }
    }
  }
};
</script>
