import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import loadable from "@loadable/component";

import "./App.css";
import plugins, { IRoute } from "./plugins";

const App: React.FC = () => {
  return (
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
  );
};

export default App;
