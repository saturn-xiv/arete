<template>
  <dashboard-layout v-bind:title="title">
    <v-flex md6>
      <v-card>
        <v-card-title primary-title>
          <h3 class="headline mb-0">{{title}}</h3>
        </v-card-title>
        <v-form>
          <v-text-field
            prepend-icon="person"
            name="name"
            v-model="name"
            :error-messages="errors.collect('name')"
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
            disabled
            type="email"
          />
        </v-form>
        <v-date-picker v-model="startup"/>
        <v-date-picker v-model="shutdown"/>
        <v-card-actions>
          <v-spacer/>
          <v-btn v-on:click="submit" color="primary">{{$t('buttons.submit')}}</v-btn>
        </v-card-actions>
      </v-card>
    </v-flex>
  </dashboard-layout>
</template>

<script>
import { get as httpGet, post as httpPost } from "@/request";
import { NOTIFICATION_ERROR, NOTIFICATION_SUCCESS } from "@/store";

export default {
  name: "vpn-user-edit",
  computed: {
    title() {
      return this.$i18n.t("ops.vpn.users.edit.title", { name: this.name });
    }
  },
  data() {
    return {
      email: null,
      name: null,
      startup: null,
      shutdown: null
    };
  },
  created() {
    httpGet(`/ops/vpn/users/${this.$route.params.id}`).then(rst => {
      this.email = rst.email;
      this.name = rst.name;
      this.startup = rst.startup;
      this.shutdown = rst.shutdown;
    });
  },
  methods: {
    async submit(e) {
      e.preventDefault();

      const isValid = await this.$validator.validate();
      if (isValid) {
        httpPost(`/ops/vpn/users/${this.$route.params.id}`, {
          name: this.name,
          startup: this.startup,
          shutdown: this.shutdown
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
