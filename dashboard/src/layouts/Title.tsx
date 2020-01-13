import React, { Component } from "react";
import { connect } from "react-redux";
import { Helmet } from "react-helmet";

import { IState as IApplicationState, ISite } from "../actions";

interface IProps {
  siteInfo: ISite;
  value: string;
}

interface IState {}

class Widget extends Component<IProps, IState> {
  public render() {
    const { value, siteInfo } = this.props;
    const title = `${value}|${siteInfo.subhead || ""}|${siteInfo.title || ""}`;
    return (
      <Helmet>
        <title>{title}</title>
      </Helmet>
    );
  }
}

export default connect(
  ({ siteInfo }: IApplicationState) => ({
    siteInfo
  }),
  {}
)(Widget);
