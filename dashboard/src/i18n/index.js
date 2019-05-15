import Vue from 'vue'
import VueI18n from 'vue-i18n'
import * as Cookies from 'js-cookie'

import zhHans from './zh-Hans'
import zhHant from './zh-Hant'
import enUS from './en-US'

Vue.use(VueI18n)


const KEY = "locale"

export const set = (l) => {
    Cookies.set(KEY, l, {
        expires: Math.pow(2, 16),
        path: '/'
    })
    localStorage.setItem(KEY, l)
}

export const get = () => {
    const it = Cookies.get(KEY) || localStorage.getItem(KEY) || 'en-US'
    return it
}

export default new VueI18n({
    locale: get(),
    messages: {
        'en-US': enUS,
        'zh-Hans': zhHans,
        'zh-Hant': zhHant,
    },
})