import { LocaleProvider, message, Spin } from 'antd';
import { Locale as AntdLocale } from 'antd/lib/locale-provider';
import * as moment from 'moment';
import * as React from 'react';
import { IntlProvider } from 'react-intl';
import * as Loadable from 'react-loadable';
import { Provider, ProviderProps } from 'react-redux';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';


import './App.css';
import { get as getLocale, ILocale } from './intl';
import Layout from './layout';
import pages from './pages';
import NoMatch from './pages/NoMatch';
import { httpGet } from './utils/request';

const Loading = () => (<Spin size="large" />);

interface IFormState {
  locale: string,
  antd: AntdLocale,
  messages: ILocale[],
}

class Widget extends React.Component<ProviderProps, IFormState> {
  constructor(props: any) {
    super(props);

    const lang = getLocale();
    moment.locale(lang.moment);

    this.state = {
      antd: lang.antd,
      locale: lang.locale,
      messages: [],
    };
  }
  public componentDidMount() {
    httpGet(`/locales/${this.state.locale}`).then((rst) => { this.setState({ messages: rst }) }).catch(message.error);
  }
  public render() {
    return (<Provider store={this.props.store} >
      <IntlProvider locale={this.state.locale} messages={this.state.messages.reduce((acc, cur) => {
        acc[cur.code] = cur.message;
        return acc;
      }, {})} >
        <LocaleProvider locale={this.state.antd}>
          <Router basename="/my" >
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
    </Provider>);
  }
}

export default Widget;
