import React from "react";
import { injectIntl, WrappedComponentProps } from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import {
  PrimaryButton,
  TextField,
  MessageBarType
} from "office-ui-fabric-react";
import { connect } from "react-redux";

import Layout from "./users/SharedLinks";
import { validate, CONSTRAIONTS } from "../../form";
import { post as httpPost } from "../../request";
import { showMessageBar, IState as IApplicationState } from "../../actions";

interface IProps {
  showMessageBar: typeof showMessageBar;
}
interface IForm {
  realName: string;
  password: string;
  email: string;
  passwordConfirmation: string;
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
      form: { email: "", realName: "", password: "", passwordConfirmation: "" }
    };
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const { history, intl, showMessageBar } = this.props;

    var msg = validate(this.state.form, {
      email: CONSTRAIONTS.email,
      realName: CONSTRAIONTS.realName,
      password: CONSTRAIONTS.password,
      passwordConfirmation: CONSTRAIONTS.passwordConfirmation
    });
    if (msg) {
      showMessageBar({ type: MessageBarType.error, messages: msg });
    } else {
      httpPost("/install", this.state.form)
        .then(() => {
          showMessageBar({
            type: MessageBarType.success,
            messages: [intl.formatMessage({ id: "flashes.success" })]
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
      <Layout title="nut.install.title">
        <form onSubmit={this.handleSubmit}>
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

const mapStateToProps = ({ siteInfo }: IApplicationState) => ({ siteInfo });

const mapDispatchToProps = { showMessageBar };

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(withRouter(Widget)));
