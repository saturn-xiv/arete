import React from "react";
import { injectIntl, WrappedComponentProps } from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import {
  PrimaryButton,
  TextField,
  MessageBarType
} from "office-ui-fabric-react";
import { connect } from "react-redux";

import Layout from "./SharedLinks";
import { validate, CONSTRAIONTS, HOME } from "../../../form";
import { post as httpPost } from "../../../request";
import { showMessageBar } from "../../../actions";

interface IProps {
  showMessageBar: typeof showMessageBar;
  action: string;
}
interface IForm {
  email: string;
}
interface IState {
  form: IForm;
}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  constructor(
    props: RouteComponentProps<any> & WrappedComponentProps & IProps
  ) {
    super(props);
    this.state = {
      form: {
        email: ""
      }
    };
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const { history, intl, action, showMessageBar } = this.props;

    var msg = validate(this.state.form, {
      email: CONSTRAIONTS.email
    });
    if (msg) {
      showMessageBar({ type: MessageBarType.error, messages: msg });
    } else {
      httpPost(
        `/users/${action}`,
        Object.assign({}, this.state.form, { home: HOME })
      )
        .then(() => {
          showMessageBar({
            type: MessageBarType.success,
            messages: [
              intl.formatMessage({ id: `nut.users.${action}.success` })
            ]
          });

          history.push("/users/sign-in");
        })
        .catch(e => {
          showMessageBar({ type: MessageBarType.error, messages: [e] });
        });
    }
  };
  public handleChange = (
    e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    var target = e.target as HTMLInputElement;
    var v: any = {};
    v[target.id] = target.value;
    var form = Object.assign({}, this.state.form, v);
    this.setState({ form });
  };
  public render() {
    const { formatMessage } = this.props.intl;

    return (
      <Layout title={`nut.users.${this.props.action}.title`}>
        <form onSubmit={this.handleSubmit}>
          <TextField
            id="email"
            required
            value={this.state.form.email}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.email" })}
          />
          <br />
          <PrimaryButton
            type="submit"
            text={formatMessage({ id: "form.buttons.submit" })}
          />
        </form>
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
