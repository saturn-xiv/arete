<template>
  <dashboard-layout v-bind:title="title">
    <v-flex md6>
      <v-card>
        <v-card-title primary-title>
          <h3 class="headline mb-0">{{title}}</h3>
        </v-card-title>
        <v-form>
          <v-text-field
            prepend-icon="perm_identity"
            name="nickName"
            v-model="nickName"
            disabled
            :label="this.$t('form.labels.nickname')"
            type="text"
          />
          <v-text-field
            prepend-icon="email"
            name="email"
            v-model="email"
            disabled
            :label="this.$t('form.labels.email')"
            type="email"
          />
          <v-text-field
            prepend-icon="person_pin"
            name="realName"
            :error-messages="errors.collect('realName')"
            v-model="realName"
            v-validate="'required'"
            :label="this.$t('form.labels.real-name')"
            type="text"
          />
          <v-text-field
            prepend-icon="credit_card"
            name="logo"
            :error-messages="errors.collect('logo')"
            v-model="logo"
            v-validate="'required'"
            :label="this.$t('form.labels.logo')"
            type="text"
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
import { get as httpGet, post as httpPost } from "@/request";

export default {
  name: "users-profile",
  computed: {
    title() {
      return this.$i18n.t("nut.users.profile.title");
    }
  },
  data() {
    return {
      email: null,
      nickName: null,
      realName: null,
      logo: null,
      alert: {}
    };
  },
  created() {
    httpGet("/users/profile").then(rst => {
      this.email = rst.email;
      this.realName = rst.realName;
      this.logo = rst.logo;
      this.nickName = rst.nickName;
    });
  },
  methods: {
    async submit(e) {
      e.preventDefault();
      this.alert = {};
      const isValid = await this.$validator.validate();
      if (isValid) {
        httpPost("/users/profile", {
          logo: this.logo,
          realName: this.realName
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
