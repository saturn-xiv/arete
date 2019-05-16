<template>
  <dashboard-layout v-bind:title="title">
    <v-flex md6>
      <v-card>
        <v-card-title primary-title>
          <h3 class="headline mb-0">{{title}}</h3>
        </v-card-title>
        <v-form>
          <v-text-field
            name="host"
            v-model="host"
            :error-messages="errors.collect('host')"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.host')"
            type="text"
          />
          <v-text-field
            name="ip"
            v-model="ip"
            :error-messages="errors.collect('ip')"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.ip')"
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
            name="device"
            v-model="device"
            :error-messages="errors.collect('device')"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.interface')"
            type="text"
          />
          <v-text-field
            name="dns"
            :error-messages="errors.collect('dns')"
            v-model="dns"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.dns')"
            type="text"
          />
          <v-text-field
            name="serverNetwork"
            v-model="serverNetwork"
            :error-messages="errors.collect('serverNetwork')"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.server.network')"
            type="text"
          />
          <v-text-field
            name="serverNetmask"
            v-model="serverNetmask"
            :error-messages="errors.collect('serverNetmask')"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.server.netmask')"
            type="text"
          />
          <v-text-field
            name="clientNetwork"
            :error-messages="errors.collect('clientNetwork')"
            v-model="clientNetwork"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.client.network')"
            type="text"
          />
          <v-text-field
            name="clientNetmask"
            :error-messages="errors.collect('clientNetmask')"
            v-model="clientNetmask"
            v-validate="'required'"
            :label="this.$t('ops.vpn.settings.client.netmask')"
            type="text"
          />
        </v-form>
        <v-card-actions>
          <v-spacer/>
          <v-icon class="mr-2" @click="fetch_files()">attach_file</v-icon>
          <v-btn v-on:click="submit" color="primary">{{$t('buttons.submit')}}</v-btn>
        </v-card-actions>
      </v-card>
    </v-flex>
    <v-dialog v-model="dialog">
      <v-card>
        <v-card-title>
          <span class="headline">{{$t('ops.vpn.dashboard.files')}}</span>
        </v-card-title>
        <v-card-text>
          <file-list :items="files"/>
        </v-card-text>
        <v-card-actions>
          <v-spacer/>
          <v-btn color="green darken-1" flat="flat" @click="dialog = false">{{$t('buttons.close')}}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
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
      dns: null,
      host: null,
      ip: null,
      device: null,
      port: null,
      clientNetwork: null,
      clientNetmask: null,
      serverNetwork: null,
      serverNetmask: null,
      files: [],
      alert: {},
      dialog: false
    };
  },
  created() {
    client.get(`/ops/vpn/`).then(rst => {
      this.serverNetwork = rst.data.server.network;
      this.serverNetmask = rst.data.server.netmask;
      this.clientNetmask = rst.data.client.netmask;
      this.clientNetwork = rst.data.client.network;
      this.dns = rst.data.dns;
      this.port = rst.data.port.toString();
      this.host = rst.data.host;
      this.ip = rst.data.ip;
      this.device = rst.data.interface;
    });
  },
  methods: {
    fetch_files() {
      client
        .get(`/ops/vpn/server`)
        .then(rst => {
          this.files = rst.data;
          this.dialog = true;
        })
        .catch(error => {
          this.alert = { ok: false, message: error.response.data };
        });
    },
    async submit(e) {
      e.preventDefault();
      this.alert = {};
      const isValid = await this.$validator.validate();
      if (isValid) {
        client
          .post(`/ops/vpn`, {
            port: parseInt(this.port),
            dns: this.dns,
            host: this.host,
            ip: this.ip,
            interface: this.device,
            server: {
              network: this.serverNetwork,
              netmask: this.serverNetmask
            },
            client: {
              network: this.clientNetwork,
              netmask: this.clientNetmask
            }
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
