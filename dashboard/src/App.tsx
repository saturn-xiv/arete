import { LocaleProvider, Spin } from 'antd'
import { Locale as AntdLocale } from 'antd/lib/locale-provider'
import * as React from 'react'
import { IntlProvider } from 'react-intl'
import * as Loadable from 'react-loadable'
import { Provider, ProviderProps } from 'react-redux'
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom'

import './App.css'
import { ILocale } from './intl'
import Layout from './layout'
import pages from './pages'
import NoMatch from './pages/NoMatch'

const Loading = () => (<Spin size="large" />)

interface IProps {
  locale: string,
  antd: AntdLocale,
  messages: ILocale[],
}

class Widget extends React.Component<ProviderProps & IProps> {
  public render() {
    return (<Provider store={this.props.store} >
      <IntlProvider locale={this.props.locale} messages={this.props.messages.reduce((acc, cur) => {
        acc[cur.code] = cur.message
        return acc
      }, {})} >
        <LocaleProvider locale={this.props.antd}>
          <Router basename="/my">
            <Layout>
              <Switch>
                {pages.map(it => (<Route
                  key={it.path}
                  path={it.path}
                  component={Loadable({ loader: it.component, loading: Loading })}
                  exact={true} />))}
                <Route component={NoMatch} />
              </Switch>
            </Layout>
          </Router>
        </LocaleProvider>
      </IntlProvider>
    </Provider>)
  }
}

export default Widget
