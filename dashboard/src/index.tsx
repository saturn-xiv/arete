import { message } from 'antd'
import * as moment from 'moment'
import * as React from 'react'
import * as ReactDOM from 'react-dom'
import { createStore } from 'redux'

import App from './App'
import { get as getLocale } from './intl'
import { rootReducers } from './reducers'
import registerServiceWorker from './registerServiceWorker'
import { httpGet } from './utils/request'

const lang = getLocale()
moment.locale(lang.moment)

const store = createStore(rootReducers)

httpGet(`/locales/${lang.locale}`)
  .then((rst) => {
    ReactDOM.render(
      (<App store={store} locale={lang.locale} antd={lang.antd} messages={rst} />),
      document.getElementById('root') as HTMLElement
    )
  })
  .catch(message.error)


registerServiceWorker()
