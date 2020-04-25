import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import loadable from "@loadable/component";

import plugins, { IMenu } from "./plugins";
import Authorized, { IRoute } from "./components/Authorized";

interface IProps {
  basename: string;
}

interface IState {}

const reduce = (a: IRoute[], i: IMenu) => {
  a.push({ component: i.component, path: i.path, authority: i.authority });
  if (i.children) {
    for (let c of i.children) {
      reduce(a, c);
    }
  }
};
{
  /* <Authorized key={it.path} item={it} /> */
}
class Component extends React.Component<IProps, IState> {
  render() {
    const { basename } = this.props;
    const routes: IRoute[] = [];
    for (let it of plugins) {
      reduce(routes, it);
    }

    return (
      <Router basename={basename}>
        <Switch>
          {routes.map((it) => (
            <Route
              exact={true}
              path={it.path}
              key={it.path}
              component={loadable(it.component)}
            />
          ))}
          <Route component={loadable(() => import("./plugins/nut/NotFound"))} />
        </Switch>
      </Router>
    );
  }
}

export default Component;
