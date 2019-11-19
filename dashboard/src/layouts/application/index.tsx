import React from "react";
import {
  injectIntl,
  FormattedMessage,
  WrappedComponentProps
} from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import { PrimaryButton, TextField } from "office-ui-fabric-react";

interface IProps {
  children: React.ReactNode;
}

interface IState {}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps<any> & IProps,
  IState
> {
  public render() {
    const { children } = this.props;

    return <div>{children}</div>;
  }
}

export default injectIntl(withRouter(Widget));
