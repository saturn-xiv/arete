import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { IntlProvider } from "react-intl";
import loadable from "@loadable/component";
import "moment-timezone";

import "./App.css";
import plugins, { IRoute } from "./plugins";
import { get as getLocale } from "./i18n";

const App: React.FC = () => {
  let locale = getLocale();
  return (
    <IntlProvider {...locale}>
      <Router basename="/my">
        <div>
          <Switch>
            {plugins
              .reduce((acc, it) => acc.concat(it.routes), new Array<IRoute>())
              .map(it => {
                const Widget = loadable(it.component, {
                  fallback: <div>loading</div>
                });
                return (
                  <Route key={it.path} path={it.path} exact={true}>
                    <Widget />
                  </Route>
                );
              })}
          </Switch>
        </div>
      </Router>
    </IntlProvider>
  );
};

export default App;
