<template>
  <v-app id="inspire">
    <v-navigation-drawer v-model="drawer" :clipped="$vuetify.breakpoint.lgAndUp" fixed app>
      <v-list dense>
        <template v-for="item in items">
          <v-layout v-if="item.heading" :key="item.heading" row align-center>
            <v-flex xs6>
              <v-subheader v-if="item.heading">{{ item.heading }}</v-subheader>
            </v-flex>
            <v-flex xs6 class="text-xs-center">
              <a href="#!" class="body-2 black--text">EDIT</a>
            </v-flex>
          </v-layout>
          <v-list-group
            v-else-if="item.children"
            :key="item.text"
            v-model="item.model"
            :prepend-icon="item.model ? item.icon : item['icon-alt']"
            append-icon
          >
            <template v-slot:activator>
              <v-list-tile>
                <v-list-tile-content>
                  <v-list-tile-title>{{ item.text }}</v-list-tile-title>
                </v-list-tile-content>
              </v-list-tile>
            </template>
            <v-list-tile v-for="(child, i) in item.children" :key="i" @click="goto(child.to)">
              <v-list-tile-action v-if="child.icon">
                <v-icon>{{ child.icon }}</v-icon>
              </v-list-tile-action>
              <v-list-tile-content>
                <v-list-tile-title>{{ child.text }}</v-list-tile-title>
              </v-list-tile-content>
            </v-list-tile>
          </v-list-group>
          <v-list-tile v-else :key="item.text" @click="goto(it.to)">
            <v-list-tile-action>
              <v-icon>{{ item.icon }}</v-icon>
            </v-list-tile-action>
            <v-list-tile-content>
              <v-list-tile-title>{{ item.text }}</v-list-tile-title>
            </v-list-tile-content>
          </v-list-tile>
        </template>
      </v-list>
    </v-navigation-drawer>
    <v-toolbar :clipped-left="$vuetify.breakpoint.lgAndUp" color="blue darken-3" dark app fixed>
      <v-toolbar-title style="width: 300px" class="ml-0 pl-3">
        <v-toolbar-side-icon @click.stop="drawer = !drawer"></v-toolbar-side-icon>
        <span class="hidden-sm-and-down">{{title}}</span>
      </v-toolbar-title>
      <v-text-field
        flat
        solo-inverted
        hide-details
        prepend-inner-icon="search"
        label="Search"
        class="hidden-sm-and-down"
      ></v-text-field>
      <v-spacer></v-spacer>
      <v-btn icon>
        <v-icon>apps</v-icon>
      </v-btn>
      <v-btn icon>
        <v-icon>notifications</v-icon>
      </v-btn>
      <v-btn icon large>
        <v-avatar size="32px" tile>
          <img :src="logo">
        </v-avatar>
      </v-btn>
    </v-toolbar>
    <v-content>
      <v-container fluid fill-height>
        <v-layout justify-center align-center>
          <slot/>
        </v-layout>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
import logo from "@/assets/logo.svg";

export default {
  computed: {
    items() {
      return [
        {
          icon: "keyboard_arrow_up",
          "icon-alt": "keyboard_arrow_down",
          text: this.$i18n.t("nut.personal.title"),
          model: false,
          children: [
            {
              icon: "format_list_bulleted",
              to: { name: "users.logs" },
              text: this.$i18n.t("nut.users.logs.title")
            },
            {
              icon: "security",
              to: { name: "users.change-password" },
              text: this.$i18n.t("nut.users.change-password.title")
            },
            {
              icon: "person",
              to: { name: "users.profile" },
              text: this.$i18n.t("nut.users.profile.title")
            }
          ]
        },
        {
          icon: "keyboard_arrow_up",
          "icon-alt": "keyboard_arrow_down",
          text: this.$i18n.t("ops.vpn.dashboard.title"),
          model: false,
          children: [
            {
              icon: "settings",
              to: { name: "ops.vpn.settings" },
              text: this.$i18n.t("ops.vpn.settings.title")
            },
            {
              icon: "format_list_bulleted",
              to: { name: "ops.vpn.logs.index" },
              text: this.$i18n.t("ops.vpn.logs.index.title")
            },
            {
              icon: "group",
              to: { name: "ops.vpn.users.index" },
              text: this.$i18n.t("ops.vpn.users.index.title")
            }
          ]
        }
      ];
    }
  },
  data() {
    return {
      logo,
      drawer: null
    };
  },
  props: {
    title: String
  },
  methods: {
    goto(to) {
      this.$router.push(to);
    }
  }
};
</script>
