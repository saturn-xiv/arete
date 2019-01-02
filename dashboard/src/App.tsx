import { Spinner, SpinnerSize } from 'office-ui-fabric-react/lib/Spinner'
import * as React from 'react'
import * as Loadable from 'react-loadable'
import { Provider } from 'react-redux'
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom'
import { createStore } from 'redux'

import Layout from './layout'
import pages from './pages'
import NoMatch from './pages/NoMatch'
import { rootReducers } from './reducers'

const store = createStore(rootReducers)

const Loading = () => <Spinner size={SpinnerSize.large} />

class Widget extends React.Component {
  public render() {
    return (<Provider store={store}>
      <Router basename="/my">
        <Layout>
          <Switch>
            {pages.map(it => <Route key={it.path} path={it.path} component={Loadable({ loader: it.component, loading: Loading })} exact={true} />)}
            <Route component={NoMatch} />
          </Switch>
        </Layout>
      </Router>
    </Provider>)
  }
}

export default Widget
