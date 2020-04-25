import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";

interface IProps {}

interface IState {}

class Component extends React.Component<
  RouteComponentProps<any> & IProps,
  IState
> {
  componentDidUpdate(prevProps: RouteComponentProps<IProps>) {
    if (this.props.location.pathname !== prevProps.location.pathname) {
      window.scrollTo(0, 0);
    }
  }

  render() {
    return null;
  }
}

export default withRouter(Component);
