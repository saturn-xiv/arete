import React, { Component } from "react";
import { connect } from "react-redux";
import { RouteComponentProps, withRouter } from "react-router";

import { get as getToken } from "../../utils/token";
import { IState as IApplicationState } from "../../actions";

interface IProps {}
interface IState {}

class Widget extends Component<RouteComponentProps<any> & IProps, IState> {
  componentDidMount() {
    const { history } = this.props;
    history.push(getToken() ? "/users/logs" : "/users/sign-in");
  }
  render() {
    return <div />;
  }
}

export default withRouter(
  connect(({ currentUser }: IApplicationState) => ({ currentUser }), {})(Widget)
);
