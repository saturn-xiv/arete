import React, { Component } from "react";
import { Layout, message } from "antd";
import { connect } from "react-redux";

import { get as httpGet } from "../utils/request";
import { refresh, IState as IApplicationState, ISite } from "../actions";

const Footer = Layout.Footer;

interface IProps {
  refresh: typeof refresh;
  siteInfo: ISite;
}

interface IState {}

class Widget extends Component<IProps, IState> {
  componentDidMount() {
    const { siteInfo, refresh } = this.props;
    if (!siteInfo.version) {
      httpGet("/about")
        .then(rst => {
          refresh(rst);
        })
        .catch(message.error);
    }
  }
  public render() {
    const { siteInfo } = this.props;
    return (
      <Footer style={{ textAlign: "center" }}>
        {siteInfo.copyright}
        &nbsp;
        {siteInfo.version}({siteInfo.build})
      </Footer>
    );
  }
}

export default connect(
  ({ siteInfo }: IApplicationState) => ({
    siteInfo
  }),
  { refresh }
)(Widget);
