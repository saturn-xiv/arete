import React from "react";
import { Link } from "react-router-dom";
import { MessageDescriptor, FormattedMessage } from "react-intl";

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
            <FormattedMessage id="nut.install.title" tagName="h1" />
            {children}
            <nav>
              <ul>
                <li>
                  <Link to="/">Home</Link>
                </li>
                <li>
                  <Link to="/install">Install</Link>
                </li>
                <li>
                  <Link to="/users/sign-in">Sign In </Link>
                </li>
                <li>
                  <Link to="/users/sign-up">Sign Up</Link>
                </li>
              </ul>
            </nav>
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
