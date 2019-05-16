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
  </dashboard-layout>
</template>

<script>
import { get as httpGet, post as httpPost } from "@/request";
import { NOTIFICATION_ERROR, NOTIFICATION_SUCCESS } from "@/store";

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
      dialog: false
    };
  },
  created() {
    httpGet(`/ops/vpn/`).then(rst => {
      this.serverNetwork = rst.server.network;
      this.serverNetmask = rst.server.netmask;
      this.clientNetmask = rst.client.netmask;
      this.clientNetwork = rst.client.network;
      this.dns = rst.dns;
      this.port = rst.port.toString();
      this.host = rst.host;
      this.ip = rst.ip;
      this.device = rst.interface;
    });
  },
  methods: {
    fetch_files() {
      httpGet(`/ops/vpn/server`)
        .then(rst => {
          this.files = rst;
          this.dialog = true;
        })
        .catch(err => {
          this.$store.commit(NOTIFICATION_ERROR, err);
        });
    },
    async submit(e) {
      e.preventDefault();

      const isValid = await this.$validator.validate();
      if (isValid) {
        httpPost(`/ops/vpn`, {
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
