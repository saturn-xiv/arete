import antdEnUS from 'antd/lib/locale-provider/en_US'
import antdZhHans from 'antd/lib/locale-provider/zh_CN'
import antdZhHant from 'antd/lib/locale-provider/zh_TW'
import * as Cookies from 'js-cookie'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'
import { addLocaleData } from 'react-intl'
import * as dataEn from 'react-intl/locale-data/en'
import * as dataZh from 'react-intl/locale-data/zh'

import { detect as detectMoment } from './components/moment'

const KEY = "locale"

export interface ILocale {
  id: number,
  code: string,
  message: string,
  createdAt: Date,
}

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
  const moment = detectMoment(locale)
  switch (locale) {
    case 'zh-Hans':
      return { moment, locale, antd: antdZhHans }
    case 'zh-Hant':
      return { moment, locale, antd: antdZhHant }
    default:
      return { moment, locale: 'en-US', antd: antdEnUS }
  }
}
