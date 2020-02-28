import React, { Component } from "react";
import { connect } from "react-redux";
import { Button, Form, Input, message } from "antd";
import {
  injectIntl,
  WrappedComponentProps,
  FormattedMessage
} from "react-intl";
import { withRouter, RouteComponentProps } from "react-router";

import { post as httpPost } from "../../utils/request";
import { signIn } from "../../actions";
import Layout from "../../layouts/application";

const FormItem = Form.Item;

interface IProps {
  signIn: typeof signIn;
}

interface IState {}

class Widget extends Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  onFinish = (values: any) => {
    const { intl, signIn, history } = this.props;
    httpPost("/users/sign-in", values)
      .then(rst => {
        signIn(rst.token);
        message.success(intl.formatMessage({ id: "flashes.success" }));
        history.push("/users/logs");
      })
      .catch(message.error);
  };

  public render() {
    const { formatMessage } = this.props.intl;

    const title = { id: "nut.install.title" };
    return (
      <Layout title={formatMessage(title)}>
        <Form
          name="nut.install"
          initialValues={{}}
          onFinish={this.onFinish}
          onFinishFailed={message.error}
        >
          <FormItem
            name="login"
            rules={[
              {
                required: true
              }
            ]}
            label={<FormattedMessage id="attributes.login" />}
          >
            <Input />
          </FormItem>
          <FormItem
            name="password"
            rules={[
              {
                required: true
              }
            ]}
            label={<FormattedMessage id="attributes.password" />}
          >
            <Input type="password" />
          </FormItem>
          <FormItem>
            <Button type="primary" htmlType="submit">
              <FormattedMessage id="buttons.submit" />
            </Button>
          </FormItem>
        </Form>
      </Layout>
    );
  }
}

export default injectIntl(connect(() => ({}), { signIn })(withRouter(Widget)));
