import React from "react";
import { Link } from "react-router-dom";
import { MessageDescriptor, FormattedMessage } from "react-intl";
import { Stack, FontIcon } from "office-ui-fabric-react";

import Layout from "../../../layouts/application";

interface IProps {
  children: React.ReactNode;
  title: MessageDescriptor;
}

class Widget extends React.Component<IProps, {}> {
  public render() {
    const { children, title } = this.props;

    return (
      <Layout title={title}>
        <div className="ms-Grid-row">
          <div className="ms-Grid-col ms-sm12 ms-md6 ms-mdPush3 ms-lg4 ms-lgPush4">
            <FormattedMessage {...title} tagName="h1" />
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

export default Widget;

// const Widget: React.FC = ({}) => {
//   return (
//     <Layout>

//     </Layout>
//   );
// };
