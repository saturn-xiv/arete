import * as React from 'react'
import * as ReactDOM from 'react-dom'
import { createStore } from 'redux'

import App from './App'
import { rootReducers } from './reducers'
import registerServiceWorker from './registerServiceWorker'

const store = createStore(rootReducers)

ReactDOM.render(
  <App store={store} />,
  document.getElementById('root') as HTMLElement
)

registerServiceWorker()
