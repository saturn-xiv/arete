import * as Cookies from 'js-cookie'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'
import { addLocaleData } from 'react-intl'
import * as dataEn from 'react-intl/locale-data/en'
import * as dataZh from 'react-intl/locale-data/zh'

import enUS from './en-US'
import zhHans from './zh-Hans'
import zhHant from './zh-Hant'

const KEY = "locale"

export const set = (l: string) => {
    Cookies.set(KEY, l, {
        expires: Math.pow(2, 16),
        path: '/'
    })
    localStorage.setItem(KEY, l)
}

export const get = () => {
    addLocaleData([
        ...dataEn,
        ...dataZh
    ])
    const locale = Cookies.get(KEY) || localStorage.getItem(KEY) || 'en-US'
    switch (locale) {
        case 'zh-Hans':
            return { moment: 'zh-cn', locale, messages: zhHans }
        case 'zh-Hant':
            return { moment: 'zh-tw', locale, messages: zhHant }
        default:
            return { moment: 'en', locale: 'en-US', messages: enUS }
    }
}
