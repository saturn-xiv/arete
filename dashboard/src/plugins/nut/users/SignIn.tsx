import React, { Component } from "react";
import { connect } from "react-redux";
import { Button, Form, Input, message } from "antd";
import {
  injectIntl,
  WrappedComponentProps,
  FormattedMessage
} from "react-intl";
import { withRouter, RouteComponentProps } from "react-router";

import { post as httpPost } from "../../../utils/request";
import { signIn } from "../../../actions";
import Layout from "../../../layouts/application";
import { LAYOUT, TAIL_LAYOUT } from "../../../components/form";

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
    const title = { id: "nut.users.sign-in.title" };
    return (
      <Layout title={formatMessage(title)}>
        <Form
          {...LAYOUT}
          name="nut.sign-in"
          initialValues={{}}
          onFinish={this.onFinish}
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
          <FormItem {...TAIL_LAYOUT}>
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
