import Vue from 'vue'
import Moment from 'vue-moment'
import Vuetify from 'vuetify'
import VeeValidate from 'vee-validate'

import FileList from './FileList'

Vue.use(VeeValidate)
Vue.use(Moment)
Vue.use(Vuetify, {
    iconfont: 'md'
})

Vue.component('file-list', FileList)