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
import { validate, CONSTRAIONTS } from "../../../form";
import { post as httpPost } from "../../../request";
import { showMessageBar, IState as IApplicationState } from "../../../actions";

interface IProps {
  showMessageBar: typeof showMessageBar;
}
interface IForm {
  login: string;
  password: string;
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
        login: "",
        password: ""
      }
    };
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const { history, intl, showMessageBar } = this.props;

    var msg = validate(this.state.form, {
      login: CONSTRAIONTS.nickName,
      password: CONSTRAIONTS.password
    });
    if (msg) {
      showMessageBar({ type: MessageBarType.error, messages: msg });
    } else {
      httpPost("/users/sign-in", Object.assign({}, this.state.form))
        .then(() => {
          showMessageBar({
            type: MessageBarType.success,
            messages: [intl.formatMessage({ id: "flashes.success" })]
          });
          // TODO
          history.push("/users/logs");
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
      <Layout title="nut.users.sign-in.title">
        <form onSubmit={this.handleSubmit}>
          <TextField
            id="login"
            required
            value={this.state.form.login}
            onChange={this.handleChange}
            label={formatMessage({ id: "nut.users.sign-in.login" })}
          />
          <TextField
            id="password"
            required
            type="password"
            value={this.state.form.password}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.password" })}
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
