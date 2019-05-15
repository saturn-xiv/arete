<template>
  <dashboard-layout v-bind:title="title">
    <v-data-table :headers="headers" :items="items" class="elevation-1">
      <template v-slot:items="props">
        <td>{{ props.item.createdAt|moment('llll') }}</td>
        <td>{{ props.item.ip }}</td>
        <td>{{ props.item.message }}</td>
      </template>
    </v-data-table>
  </dashboard-layout>
</template>

<script>
import client from "@/request";

export default {
  name: "users-logs",
  data() {
    return {
      items: []
    };
  },
  created() {
    client.get(`/users/logs?limit=${1 << 10}`).then(rst => {
      this.items = rst.data;
    });
  },
  computed: {
    headers() {
      return [
        {
          text: this.$i18n.t("form.labels.created-at"),
          value: "createdAt"
        },
        {
          text: this.$i18n.t("form.labels.ip"),
          value: "ip"
        },
        {
          text: this.$i18n.t("form.labels.message"),
          value: "message"
        }
      ];
    },
    title() {
      return this.$i18n.t("nut.users.logs.title");
    }
  }
};
</script>
