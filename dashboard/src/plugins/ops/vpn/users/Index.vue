<template>
  <dashboard-layout v-bind:title="title">
    <v-data-table :headers="headers" :items="items" class="elevation-1">
      <template v-slot:items="props">
        <td>{{ props.item.updatedAt|moment('llll') }}</td>
        <td>{{ props.item.name }}</td>
        <td>{{ props.item.email }}</td>
        <td>{{ $t(`form.labels.${props.item.online}`) }}</td>
        <td>[{{ props.item.startup }}, {{ props.item.shutdown }}]</td>
        <td>
          <v-icon
            small
            class="mr-2"
            @click="$router.push({name:'ops.vpn.users.edit', params:{id: props.item.id}})"
          >edit</v-icon>
          <v-icon small class="mr-2" @click="fetch_files(props.item.id)">attach_file</v-icon>
        </td>
      </template>
    </v-data-table>
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
    <v-btn
      fab
      bottom
      right
      color="pink"
      dark
      fixed
      @click="$router.push({name:'ops.vpn.users.new'})"
    >
      <v-icon>add</v-icon>
    </v-btn>
  </dashboard-layout>
</template>

<script>
import client from "@/request";

export default {
  name: "ops-vpn-users",
  data() {
    return {
      items: [],
      files: [],
      dialog: false,
      alert: {}
    };
  },
  created() {
    client.get(`/ops/vpn/users`).then(rst => {
      this.items = rst.data;
    });
  },
  methods: {
    fetch_files(id) {
      client
        .get(`/ops/vpn/client/${id}`)
        .then(rst => {
          this.files = rst.data;
          this.dialog = true;
        })
        .catch(error => {
          this.alert = { ok: false, message: error.response.data };
        });
    }
  },
  computed: {
    headers() {
      return [
        {
          text: this.$i18n.t("form.labels.updated-at"),
          value: "updatedAt"
        },
        {
          text: this.$i18n.t("form.labels.real-name"),
          value: "name"
        },
        {
          text: this.$i18n.t("form.labels.email"),
          value: "email"
        },
        {
          text: this.$i18n.t("ops.vpn.form.labels.user.online"),
          value: "online"
        },
        {
          text: this.$i18n.t("form.labels.range.date"),
          value: "dates"
        },
        {
          text: this.$i18n.t("buttons.actions"),
          value: "actions"
        }
      ];
    },
    title() {
      return this.$i18n.t("ops.vpn.users.index.title");
    }
  }
};
</script>
