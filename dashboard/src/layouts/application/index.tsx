import React from "react";
import Helmet from "react-helmet";

interface IProps {
  children: React.ReactNode;
  title: string;
}

interface IState {}

class Widget extends React.Component<IProps, IState> {
  public render() {
    return (
      <div className="ms-Grid">
        {this.props.children}
        <Helmet>
          <title>{this.props.title}</title>
        </Helmet>
      </div>
    );
  }
}

export default Widget;
