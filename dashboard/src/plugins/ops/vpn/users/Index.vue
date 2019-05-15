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
          <v-icon
            small
            class="mr-2"
            @click="$router.push({name:'ops.vpn.users.edit', params:{id: props.item.id}})"
          >file_download</v-icon>
        </td>
      </template>
    </v-data-table>
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
      items: []
    };
  },
  created() {
    client.get(`/ops/vpn/users`).then(rst => {
      this.items = rst.data;
    });
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
