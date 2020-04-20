import React, { Component } from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import loadable from "@loadable/component";

import routes from "./plugins";

interface IProps {
  basename: string;
}

interface IState {}

class Widget extends Component<IProps, IState> {
  render() {
    const { basename } = this.props;
    return (
      <Router basename={basename}>
        <Switch>
          {routes.map((it) => (
            <Route
              key={it.path}
              path={it.path}
              exact={true}
              component={loadable(it.component)}
            />
          ))}
          <Route
            path="/"
            exact={true}
            component={loadable(() => import("./plugins/nut/Home"))}
          />
          <Route component={loadable(() => import("./plugins/nut/NotFound"))} />
        </Switch>
      </Router>
    );
  }
}

export default Widget;
