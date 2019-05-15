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
            name="nickname"
            v-model="nickname"
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
import client from "@/request";

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
      nickname: null,
      realName: null,
      logo: null,
      alert: {}
    };
  },
  created() {
    client.get("/users/profile").then(rst => {
      this.email = rst.data.email;
      this.realName = rst.data.realName;
      this.logo = rst.data.logo;
      this.nickname = rst.data.nickname;
    });
  },
  methods: {
    async submit(e) {
      e.preventDefault();
      this.alert = {};
      const isValid = await this.$validator.validate();
      if (isValid) {
        client
          .post("/users/profile", {
            logo: this.logo,
            realName: this.realName
          })
          .then(() => {
            this.alert = { ok: true, message: this.$i18n.t("flashes.success") };
          })
          .catch(error => {
            this.alert = { ok: false, message: error.response.data };
          });
      }
    }
  }
};
</script>
