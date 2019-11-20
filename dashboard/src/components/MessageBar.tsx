import React from "react";
import { injectIntl, WrappedComponentProps } from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import {
  MessageBarType,
  BaseButton,
  Button,
  MessageBar
} from "office-ui-fabric-react";
import Moment from "react-moment";
import { connect } from "react-redux";

import { DATETIME_FORMAT } from "../form";
import { hideMessageBar, IState as IApplicationState } from "../actions";

export interface IMessageBar {
  type?: MessageBarType;
  messages?: string[];
}

interface IProps {
  content: IMessageBar;
  hide: typeof hideMessageBar;
}
interface IState {}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  public handleDismiss = (
    e?: React.MouseEvent<HTMLElement | BaseButton | Button>
  ) => {
    const { hide } = this.props;
    hide();
  };
  public render() {
    const { content } = this.props;
    return content && content.type && content.messages ? (
      <MessageBar onDismiss={this.handleDismiss} messageBarType={content.type}>
        <Moment format={DATETIME_FORMAT} />
        <ol>
          {content.messages.map((it, i) => (
            <li key={i}>{it}</li>
          ))}
        </ol>
      </MessageBar>
    ) : (
      <div />
    );
  }
}

const mapStateToProps = ({ messageBar }: IApplicationState) => ({
  content: messageBar
});

const mapDispatchToProps = { hide: hideMessageBar };

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(withRouter(Widget)));
