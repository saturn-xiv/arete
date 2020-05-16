import React from "react";
import { RouteComponentProps } from "react-router";

interface IProps {}
interface IState {}

class Component extends React.Component<
  RouteComponentProps<any> & IProps,
  IState
> {
  render() {
    return <div>install</div>;
  }
}

export default Component;