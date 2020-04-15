import React, { Component } from "react";
import { Button } from "antd";
import {
  FormattedMessage,
  injectIntl,
  WrappedComponentProps
} from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";

import Layout from "../../layouts/application";
import BROKEN_LINK from "../../assets/broken-link.svg";

interface IProps {}

interface IState {}

class Widget extends Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  render() {
    const { history, intl } = this.props;
    const title = intl.formatMessage({ id: "nut.not-found.title" });
    return (
      <Layout title={title}>
        <img src={BROKEN_LINK} alt={title} />
        <Button type="link" onClick={() => history.push("/")}>
          <FormattedMessage id="nut.not-found.go-home" />
        </Button>
      </Layout>
    );
  }
}

export default withRouter(injectIntl(Widget));
