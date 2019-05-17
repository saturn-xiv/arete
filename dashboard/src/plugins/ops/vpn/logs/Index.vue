<template>
  <dashboard-layout v-bind:title="title">
    <v-data-table :headers="headers" :items="items" class="elevation-1">
      <template v-slot:items="props">
        <td>
          <timestamp :value="props.item.openedAt"/>
        </td>
        <td>
          <timestamp v-if="props.item.closedAt" :value="props.item.closedAt"/>
        </td>
        <td>{{ props.item.remoteIp }}:{{ props.item.remotePort }}</td>
        <td>{{ props.item.trustedIp }}:{{ props.item.trustedPort }}</td>
        <td>{{ props.item.received || 0 }}bytes</td>
        <td>{{ props.item.send || 0}}bytes</td>
      </template>
    </v-data-table>
  </dashboard-layout>
</template>

<script>
import { get as httpGet } from "@/request";

export default {
  name: "ops-vpn-logs",
  data() {
    return {
      items: []
    };
  },
  created() {
    httpGet(`/ops/vpn/logs?limit=${1 << 10}`).then(rst => {
      this.items = rst;
    });
  },
  computed: {
    headers() {
      return [
        {
          text: this.$i18n.t("ops.vpn.form.labels.log.opened-at"),
          value: "opened_at"
        },
        {
          text: this.$i18n.t("ops.vpn.form.labels.log.closed-at"),
          value: "closed_at"
        },
        {
          text: this.$i18n.t("ops.vpn.form.labels.log.remote"),
          value: "remote"
        },
        {
          text: this.$i18n.t("ops.vpn.form.labels.log.trusted"),
          value: "trusted"
        },
        {
          text: this.$i18n.t("ops.vpn.form.labels.log.received"),
          value: "receive"
        },
        {
          text: this.$i18n.t("ops.vpn.form.labels.log.send"),
          value: "send"
        }
      ];
    },
    title() {
      return this.$i18n.t("ops.vpn.logs.index.title");
    }
  }
};
</script>
