<template>
  <v-data-table :headers="headers" :items="items" :expand="expand" item-key="path">
    <template v-slot:items="props">
      <tr @click="props.expanded = !props.expanded">
        <td>{{ props.item.path }}</td>
        <td>0{{ props.item.mode.toString(8) }}</td>
      </tr>
    </template>
    <template v-slot:expand="props">
      <v-card flat>
        <v-card-text>
          <pre>{{props.item.content}}</pre>
        </v-card-text>
      </v-card>
    </template>
  </v-data-table>
</template>

<script>
export default {
  name: "file-list",
  props: {
    items: Array
  },
  data() {
    return {
      expand: false
    };
  },
  computed: {
    headers() {
      return [
        {
          text: this.$i18n.t("form.labels.file.path"),
          value: "path"
        },
        {
          text: this.$i18n.t("form.labels.file.mode"),
          value: "mode"
        }
      ];
    }
  }
};
</script>
