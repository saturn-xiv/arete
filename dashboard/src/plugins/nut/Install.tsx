import React from "react";
import { injectIntl, WrappedComponentProps } from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import {
  PrimaryButton,
  TextField,
  MessageBarType,
  BaseButton,
  Button,
  MessageBar
} from "office-ui-fabric-react";

import Layout from "./users/SharedLinks";
import { validate, CONSTRAIONTS, IMessage } from "../../form";

interface IProps {}
interface IForm {
  realName: string;
  password: string;
  email: string;
  passwordConfirmation: string;
}
interface IState {
  form: IForm;
  message?: IMessage;
}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps<any> & IProps,
  IState
> {
  constructor(
    props: RouteComponentProps<any> & WrappedComponentProps<any> & IProps
  ) {
    super(props);
    this.state = {
      form: { email: "", realName: "", password: "", passwordConfirmation: "" }
    };
  }

  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    var msg = validate(this.state.form, {
      email: CONSTRAIONTS.email,
      realName: CONSTRAIONTS.realName,
      password: CONSTRAIONTS.password,
      passwordConfirmation: CONSTRAIONTS.passwordConfirmation
    });
    if (msg) {
      this.setState({ message: { type: MessageBarType.error, body: msg } });
    } else {
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
  public handleDismiss = (
    e?: React.MouseEvent<HTMLElement | BaseButton | Button>
  ) => {
    this.setState({ message: undefined });
  };
  public render() {
    const { formatMessage } = this.props.intl;

    return (
      <Layout title={{ id: "nut.install.title" }}>
        <form onSubmit={this.handleSubmit}>
          {this.state.message && (
            <MessageBar
              onDismiss={this.handleDismiss}
              messageBarType={this.state.message.type}
            >
              <ol>
                {this.state.message.body.map((it, i) => (
                  <li key={i}>{it}</li>
                ))}
              </ol>
            </MessageBar>
          )}
          <TextField
            id="email"
            required
            value={this.state.form.email}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.email" })}
          />
          <TextField
            id="realName"
            required
            value={this.state.form.realName}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.real-name" })}
          />
          <TextField
            id="password"
            required
            type="password"
            value={this.state.form.password}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.password" })}
          />
          <TextField
            id="passwordConfirmation"
            required
            type="password"
            value={this.state.form.passwordConfirmation}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.password-confirmation" })}
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

export default injectIntl(withRouter(Widget));
