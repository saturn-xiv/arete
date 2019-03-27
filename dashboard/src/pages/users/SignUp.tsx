import { Form, Input, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { RouteComponentProps, withRouter } from 'react-router'

import { formItemLayout } from '../../components/form'
import Submit from '../../components/form/Submit'
import { HOME } from '../../utils'
import { graphql } from '../../utils/request'
import Layout from './SharedLinks'

const FormItem = Form.Item

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public comparePasswords = (_: any, value: string, callback: (m?: string) => void) => {
    const { form, intl } = this.props
    if (value && value !== form.getFieldValue('password')) {
      callback(intl.formatMessage({ id: "form.validations.password-confirmation" }))
    } else {
      callback()
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, history, intl } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        graphql({
          query: `mutation ($realName: String!, $nickName: String!, $email: String!, $password: String!, $home: String!) {
            userSignUp(realName: $realName, nickName: $nickName, email: $email, password: $password, home: $home)
          }`,
          variables: Object.assign({}, values, { home: HOME })
        }, () => {
          message.success(intl.formatMessage({ id: "nut.users.confirm.success" }))
          history.push("/users/sign-in")
        })
      }
    })
  }
  public render() {
    const { formatMessage } = this.props.intl
    const { getFieldDecorator } = this.props.form

    return (<Layout title="nut.users.sign-up.title">
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.email" />}>
          {
            getFieldDecorator('email', {
              rules: [
                {
                  message: formatMessage({ id: "form.validations.email" }),
                  required: true,
                  type: 'email',
                }
              ]
            })(<Input />)
          }
        </FormItem>
        <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.nick-name" />}>
          {
            getFieldDecorator('nickName', {
              rules: [
                {
                  max: 32,
                  message: formatMessage({ id: "form.validations.nick-name" }),
                  min: 2,
                  required: true,
                }
              ]
            })(<Input />)
          }
        </FormItem>
        <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.username" />}>
          {
            getFieldDecorator('realName', {
              rules: [
                {
                  max: 32,
                  message: formatMessage({ id: "form.validations.username" }),
                  min: 2,
                  required: true,
                }
              ]
            })(<Input />)
          }
        </FormItem>
        <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.password" />}>
          {
            getFieldDecorator('password', {
              rules: [
                {
                  max: 32,
                  message: formatMessage({ id: "form.validations.password" }),
                  min: 6,
                  required: true,
                }
              ]
            })(<Input type="password" />)
          }
        </FormItem>
        <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.password-confirmation" />}>
          {
            getFieldDecorator('passwordConfirmation', {
              rules: [
                {
                  message: formatMessage({ id: "form.validations.required" }),
                  required: true,
                },
                {
                  validator: this.comparePasswords,
                },
              ]
            })(<Input type="password" />)
          }
        </FormItem>
        <Submit />
      </Form>
    </Layout>)
  }
}


export default withRouter(injectIntl(Form.create()(Widget)))
