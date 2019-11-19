import React from "react";
import { injectIntl, WrappedComponentProps } from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import { PrimaryButton, TextField } from "office-ui-fabric-react";

import Layout from "./users/SharedLinks";
import { validate, CONSTRAIONTS } from "../../form";

interface IProps {}
interface IState {
  realName: string;
  password: string;
  email: string;
  passwordConfirmation: string;
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
      email: "",
      realName: "",
      password: "",
      passwordConfirmation: ""
    };
  }

  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    var msg = validate(this.state, {
      email: CONSTRAIONTS.email,
      realName: CONSTRAIONTS.realName,
      password: CONSTRAIONTS.password,
      passwordConfirmation: CONSTRAIONTS.passwordConfirmation
    });
    console.log(msg);
    // const { form, history, intl } = this.props;
    // form.validateFields((err, values) => {
    //   if (!err) {
    //     graphql(
    //       {
    //         query: `mutation ($realName: String!, $email: String!, $password: String!) {
    //         install(realName: $realName, email: $email, password: $password)
    //       }`,
    //         variables: values
    //       },
    //       () => {
    //         message.success(intl.formatMessage({ id: "flashes.success" }));
    //         history.push("/users/sign-in");
    //       }
    //     );
    //   }
    // });
  };
  public handleChange = (
    e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    var target = e.target as HTMLInputElement;
    var v: any = {};
    v[target.id] = target.value;
    this.setState(v);
  };
  public render() {
    const { formatMessage } = this.props.intl;

    return (
      <Layout title={{ id: "nut.install.title" }}>
        <form onSubmit={this.handleSubmit}>
          <TextField
            id="email"
            required
            value={this.state.email}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.email" })}
          />
          <TextField
            id="realName"
            required
            value={this.state.realName}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.real-name" })}
          />
          <TextField
            id="password"
            required
            type="password"
            value={this.state.password}
            onChange={this.handleChange}
            label={formatMessage({ id: "form.fields.password" })}
          />
          <TextField
            id="passwordConfirmation"
            required
            type="password"
            value={this.state.passwordConfirmation}
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
