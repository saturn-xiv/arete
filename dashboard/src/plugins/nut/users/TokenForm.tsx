import React from "react";
import { injectIntl, WrappedComponentProps } from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import { connect } from "react-redux";
import { MessageBarType } from "office-ui-fabric-react";

import Layout from "../../../layouts/application";
import { patch as httpPatch } from "../../../request";
import { showMessageBar } from "../../../actions";
import MessageBar from "../../../components/MessageBar";

interface IProps {
  showMessageBar: typeof showMessageBar;
  action: string;
}
interface IState {}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  public componentDidMount() {
    const { match, action, intl, history, showMessageBar } = this.props;
    httpPatch(`/users/${action}/${match.params.token}`, {})
      .then(() => {
        showMessageBar({
          type: MessageBarType.success,
          messages: [intl.formatMessage({ id: `nut.users.${action}.success` })]
        });
      })
      .catch(e => {
        showMessageBar({ type: MessageBarType.error, messages: [e] });
      })
      .finally(() => {
        history.push("/users/sign-in");
      });
  }
  public render() {
    return (
      <Layout
        title={this.props.intl.formatMessage({
          id: `nut.users.${this.props.action}.title`
        })}
      >
        <div className="ms-Grid-row">
          <div className="ms-Grid-col ms-sm12 ms-md6 ms-mdPush3 ms-lg4 ms-lgPush4">
            <MessageBar />
          </div>
        </div>
      </Layout>
    );
  }
}

const mapStateToProps = () => ({});

const mapDispatchToProps = { showMessageBar };

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(withRouter(Widget)));
