import Vue from 'vue'

import App from './App.vue'
import {
  router
} from './plugins'
import i18n from './i18n'
import store from './store'
import './layouts'
import './components'

import './main.css'


Vue.config.productionTip = false

new Vue({
  i18n,
  router,
  store,
  render: h => h(App),
}).$mount('#app')