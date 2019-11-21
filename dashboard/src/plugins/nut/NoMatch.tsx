import React from "react";
import { Image, MessageBar, MessageBarType } from "office-ui-fabric-react";
import { RouteComponentProps, withRouter } from "react-router";
import {
  injectIntl,
  FormattedMessage,
  WrappedComponentProps
} from "react-intl";

import oops from "../../assets/oops.svg";
import Layout from "../../layouts/application";

interface IProps {}

interface IState {}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  public render() {
    const { intl } = this.props;
    const title = "flashes.not-found";
    return (
      <Layout title={intl.formatMessage({ id: title })}>
        <div className="ms-Grid-row">
          <div className="ms-Grid-col ms-sm12 ms-md6 ms-mdPush3 ms-lg4 ms-lgPush4">
            <MessageBar messageBarType={MessageBarType.error}>
              <FormattedMessage id={title} />
            </MessageBar>
            <br />
            <Image src={oops} />
          </div>
        </div>
      </Layout>
    );
  }
}

export default injectIntl(withRouter(Widget));
