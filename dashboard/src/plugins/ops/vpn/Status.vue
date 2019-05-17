<template>
  <dashboard-layout v-bind:title="title">
    <v-data-table :headers="headers" :items="items" class="elevation-1">
      <template v-slot:items="props">
        <td>
          <json-text :value="props.item.address"/>}
        </td>
        <td>
          <json-text :value="props.item.status"/>}
        </td>
      </template>
    </v-data-table>
  </dashboard-layout>
</template>

<script>
import { get as httpGet } from "@/request";

export default {
  name: "ops-vpn-status",
  data() {
    return {
      items: []
    };
  },
  created() {
    httpGet(`/ops/vpn/status`).then(rst => {
      this.items = rst.host;
    });
  },
  computed: {
    headers() {
      return [
        {
          text: this.$i18n.t("ops.vpn.status.addresses"),
          sortable: false,
          value: "addresses"
        },
        {
          text: this.$i18n.t("ops.vpn.status.status"),
          sortable: false,
          value: "status"
        }
      ];
    },
    title() {
      return this.$i18n.t("ops.vpn.status.title");
    }
  }
};
</script>
