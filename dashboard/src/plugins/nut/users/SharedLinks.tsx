import React from "react";
import { Link } from "react-router-dom";
import { Stack, FontIcon } from "office-ui-fabric-react";
import {
  injectIntl,
  FormattedMessage,
  WrappedComponentProps
} from "react-intl";
import { RouteComponentProps, withRouter } from "react-router";

import Layout from "../../../layouts/application";
import MessageBar from "../../../components/MessageBar";

interface IProps {
  children: React.ReactNode;
  title: string;
}

interface IState {}

class Widget extends React.Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  public render() {
    const { children, title, intl } = this.props;

    return (
      <Layout title={intl.formatMessage({ id: title })}>
        <div className="ms-Grid-row">
          <div className="ms-Grid-col ms-sm12 ms-md6 ms-mdPush3 ms-lg4 ms-lgPush4">
            <FormattedMessage id={title} tagName="h1" />
            <MessageBar />
            {children}
            <br />
            <Stack>
              {[
                {
                  icon: "Signin",
                  label: "users.sign-in",
                  to: "/users/sign-in"
                },
                {
                  icon: "TemporaryUser",
                  label: "users.sign-up",
                  to: "/users/sign-up"
                },
                {
                  icon: "PasswordField",
                  label: "users.forgot-password",
                  to: "/users/forgot-password"
                },
                {
                  icon: "MailCheck",
                  label: "users.confirm",
                  to: "/users/confirm"
                },
                { icon: "Unlock", label: "users.unlock", to: "/users/unlock" },
                {
                  icon: "Comment",
                  label: "leave-words.new",
                  to: "/leave-words/new"
                }
              ].map(it => (
                <Stack.Item key={it.to}>
                  <FontIcon iconName={it.icon} /> &nbsp;
                  <Link to={it.to}>
                    <FormattedMessage id={`nut.${it.label}.title`} />
                  </Link>
                </Stack.Item>
              ))}
            </Stack>
          </div>
        </div>
      </Layout>
    );
  }
}

export default injectIntl(withRouter(Widget));
