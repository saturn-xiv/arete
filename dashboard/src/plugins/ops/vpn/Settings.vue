<template>
  <dashboard-layout v-bind:title="title">
    <v-flex md6>
      <v-card>
        <v-card-title primary-title>
          <h3 class="headline mb-0">{{title}}</h3>
        </v-card-title>
        <v-form>
          <v-text-field
            name="lan"
            v-model="lan"
            :error-messages="errors.collect('lan')"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.network')"
            type="text"
          />
          <v-text-field
            name="port"
            :error-messages="errors.collect('port')"
            v-model="port"
            v-validate="'required'"
            :label="this.$t('form.labels.port')"
            type="text"
          />
          <v-text-field
            name="dns1"
            :error-messages="errors.collect('dns1')"
            v-model="dns1"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.dns1')"
            type="text"
          />
          <v-text-field
            name="dns2"
            :error-messages="errors.collect('dns2')"
            v-model="dns2"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.dns2')"
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
  name: "vpn-settings",
  computed: {
    title() {
      return this.$i18n.t("ops.vpn.settings.title");
    }
  },
  data() {
    return {
      lan: null,
      dns1: null,
      dns2: null,
      port: null,
      alert: {}
    };
  },
  created() {
    client.get(`/ops/vpn/`).then(rst => {
      this.lan = rst.data.lan;
      this.dns1 = rst.data.dns1;
      this.dns2 = rst.data.dns2;
      this.port = rst.data.port.toString();
    });
  },
  methods: {
    async submit(e) {
      e.preventDefault();
      this.alert = {};
      const isValid = await this.$validator.validate();
      if (isValid) {
        client
          .post(`/ops/vpn`, {
            lan: this.lan,
            dns1: this.dns1,
            dns2: this.dns2,
            port: parseInt(this.port)
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
