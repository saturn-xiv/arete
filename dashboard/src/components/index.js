import Vue from 'vue'
import Moment from 'vue-moment'
import Vuetify from 'vuetify'
import VeeValidate from 'vee-validate'

import FileList from './FileList'
import JsonText from './JsonText'
import Timestamp from './Timestamp'

Vue.use(VeeValidate)
Vue.use(Moment)
Vue.use(Vuetify, {
    iconfont: 'md'
})

Vue.component('file-list', FileList)
Vue.component('json-text', JsonText)
Vue.component('timestamp', Timestamp)