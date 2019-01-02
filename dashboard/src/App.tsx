import * as moment from 'moment'
import { Spinner, SpinnerSize } from 'office-ui-fabric-react/lib/Spinner'
import * as React from 'react'
import { IntlProvider } from 'react-intl'
import * as Loadable from 'react-loadable'
import { Provider } from 'react-redux'
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom'
import { createStore } from 'redux'

import './App.css';
import { get as getLocale } from './intl'
import Layout from './layout'
import pages from './pages'
import NoMatch from './pages/NoMatch'
import { rootReducers } from './reducers'

const Loading = () => <Spinner size={SpinnerSize.large} />

const locale = getLocale()
moment.locale(locale.moment)
const store = createStore(rootReducers)

class Widget extends React.Component {
  public render() {
    return (<Provider store={store}>
      <IntlProvider locale={locale.locale} messages={locale.messages}>
        <Router basename="/my">
          <Layout>
            <Switch>
              {pages.map(it => <Route key={it.path} path={it.path} component={Loadable({ loader: it.component, loading: Loading })} exact={true} />)}
              <Route component={NoMatch} />
            </Switch>
          </Layout>
        </Router>
      </IntlProvider>
    </Provider>)
  }
}

export default Widget
