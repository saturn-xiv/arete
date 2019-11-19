import React from "react";
import { MessageDescriptor } from "react-intl";

interface IProps {
  children: React.ReactNode;
  title: MessageDescriptor;
}

interface IState {}

class Widget extends React.Component<IProps, IState> {
  public render() {
    const { children } = this.props;

    return <div className="ms-Grid">{children}</div>;
  }
}

export default Widget;
