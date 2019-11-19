import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { IntlProvider } from "react-intl";
import { Provider } from "react-redux";
import { FontIcon } from "office-ui-fabric-react";
import loadable from "@loadable/component";
import "moment-timezone";

import "./App.css";
import plugins, { IRoute } from "./plugins";
import { get as getLocale } from "./i18n";
import store from "./store";

const App: React.FC = () => {
  let locale = getLocale();
  return (
    <IntlProvider {...locale}>
      <Provider store={store}>
        <Router basename="/my">
          <Switch>
            {plugins
              .reduce((acc, it) => acc.concat(it.routes), new Array<IRoute>())
              .map(it => {
                const Widget = loadable(it.component, {
                  fallback: <FontIcon iconName="Refresh" />
                });
                return (
                  <Route key={it.path} path={it.path} exact={true}>
                    <Widget />
                  </Route>
                );
              })}
          </Switch>
        </Router>
      </Provider>
    </IntlProvider>
  );
};

export default App;
