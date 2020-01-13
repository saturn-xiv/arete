import React, { Component } from "react";
import { connect } from "react-redux";
import { Button, Form, Input, message } from "antd";
import { FormComponentProps } from "antd/lib/form/Form";
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
  FormComponentProps &
    RouteComponentProps<any> &
    WrappedComponentProps &
    IProps,
  IState
> {
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const { form, intl, signIn, history } = this.props;
    form.validateFields((err, values) => {
      if (!err) {
        httpPost("/users/sign-in", values)
          .then(rst => {
            signIn(rst.token);
            message.success(intl.formatMessage({ id: "flashes.success" }));
            history.push("/users/logs");
          })
          .catch(message.error);
      }
    });
  };
  public render() {
    const { formatMessage } = this.props.intl;
    const { getFieldDecorator } = this.props.form;
    const title = { id: "nut.install.title" };
    return (
      <Layout title={formatMessage(title)}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem label={<FormattedMessage id="attributes.login" />}>
            {getFieldDecorator("login", {
              rules: [
                {
                  required: true
                }
              ]
            })(<Input />)}
          </FormItem>
          <FormItem label={<FormattedMessage id="attributes.password" />}>
            {getFieldDecorator("password", {
              rules: [
                {
                  required: true
                }
              ]
            })(<Input type="password" />)}
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

export default Form.create()(
  injectIntl(connect(() => ({}), { signIn })(withRouter(Widget)))
);
