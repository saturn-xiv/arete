import Vue from 'vue'
import Vuetify from 'vuetify'
import Vuex from 'vuex'
import VueI18n from 'vue-i18n'

import App from './App.vue'
import {
  router
} from './plugins'
import './layouts'

import './main.css'

Vue.use(Vuetify)
Vue.use(Vuex)
Vue.use(VueI18n)

Vue.config.productionTip = false

new Vue({
  router,
  render: h => h(App),
}).$mount('#app')