import React from "react";
import { Route, RouteComponentProps, Redirect } from "react-router-dom";
import { connect } from "react-redux";
import loadable from "@loadable/component";

import { IUser, IState as IApplicationState } from "../actions";

export interface IRoute {
  component: any;
  path: string;
  authority?: string[];
}

interface IProps {
  currentUser: IUser;
  item: IRoute;
}
interface IState {}

class Component extends React.Component<IProps, IState> {
  render() {
    const { item, currentUser } = this.props;
    // console.log("auth", route);
    // const Y = loadable(route.component);
    // const N = ({ location }: RouteComponentProps<any>) => (
    //   <Redirect
    //     to={{ pathname: "/users/sign-in", state: { from: location } }}
    //   />
    // );
    // const options = { exact: true };

    // if (route.authority) {
    //   if (route.authority.length > 0) {
    //     if (currentUser.uid && currentUser.roles) {
    //       for (var it in route.authority) {
    //         if (currentUser.roles.includes(it)) {
    //           return <Route {...options} render={() => <Y />} />;
    //         }
    //       }
    //     }
    //     return <Route {...options} render={N} />;
    //   }
    //   return currentUser.uid ? (
    //     <Route {...options} render={() => <Y />} />
    //   ) : (
    //     <Route {...options} render={N} />
    //   );
    // }

    return <Route render={() => <div>{item.path}</div>} />;
  }
}

export default connect(
  ({ currentUser }: IApplicationState) => ({
    currentUser,
  }),
  {}
)(Component);
