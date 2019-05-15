<template>
  <v-app id="inspire">
    <v-content>
      <v-container fluid fill-height>
        <v-layout align-center justify-center>
          <v-flex xs12 sm8 md4>
            <v-card class="elevation-12">
              <v-toolbar dark color="primary">
                <v-toolbar-title>{{ title }}</v-toolbar-title>
                <v-spacer/>
                <v-toolbar-items class="hidden-sm-and-down">
                  <v-btn v-on:click="goto('home')" icon large target="_blank">
                    <v-icon large>dashboard</v-icon>
                  </v-btn>
                  <v-btn icon large href="/" target="_blank">
                    <v-icon large>home</v-icon>
                  </v-btn>
                </v-toolbar-items>
              </v-toolbar>
              <v-card-text>
                <slot/>
                <v-chip v-on:click="goto(it.to)" :key="it.to" v-for="it in links">
                  <v-avatar>
                    <v-icon>{{it.icon}}</v-icon>
                  </v-avatar>
                  {{$t(`nut.${it.to}.title`)}}
                </v-chip>
              </v-card-text>
              <v-card-actions>
                <v-spacer/>
                <v-btn v-on:click="onSubmit" color="primary">{{$t('buttons.submit')}}</v-btn>
              </v-card-actions>
            </v-card>
            <notification-bar :alert="alert"/>
          </v-flex>
        </v-layout>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
export default {
  name: "application-layout",
  data() {
    return {
      links: [
        { to: "users.sign-in", icon: "security" },
        { to: "users.sign-up", icon: "how_to_reg" }
      ]
    };
  },
  props: {
    title: String,
    alert: Object,
    onSubmit: Function
  },
  methods: {
    goto(to) {
      this.$router.push({ name: to });
    }
  }
};
</script>

<style scoped>
</style>
