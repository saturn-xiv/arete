import React from "react";
import { injectIntl, WrappedComponentProps } from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";
import { PrimaryButton, TextField } from "office-ui-fabric-react";

import Layout from "./users/SharedLinks";

interface IProps {}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps<any> & IProps,
  {}
> {
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    console.log("aaa");
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
  public render() {
    const { formatMessage } = this.props.intl;

    return (
      <Layout title={{ id: "nut.install.title" }}>
        <form onSubmit={this.handleSubmit}>
          <TextField label={formatMessage({ id: "form.fields.email" })} />
          <TextField label={formatMessage({ id: "form.fields.real-name" })} />
          <TextField label={formatMessage({ id: "form.fields.password" })} />
          <TextField
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
