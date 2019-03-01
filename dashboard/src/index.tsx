import * as moment from 'moment'
import * as React from 'react'
import * as ReactDOM from 'react-dom'
import { createStore } from 'redux'

import App, { IMessage } from './App'
import { get as getLocale } from './intl'
import { rootReducers } from './reducers'
import registerServiceWorker from './registerServiceWorker'
import { graphql } from './utils/request'

const lang = getLocale()
moment.locale(lang.moment)

const store = createStore(rootReducers)

graphql({
  query: `
query ($lang: String!) {
  listLocaleByLang(lang: $lang){ code, message }
}`, variables: { lang: lang.locale }
}, (rst: {
  listLocaleByLang: IMessage[],
}) => {
    ReactDOM.render(
      (<App store={store} locale={lang.locale} antd={lang.antd} messages={rst.listLocaleByLang} />),
      document.getElementById('root') as HTMLElement
    )
  })



registerServiceWorker()
