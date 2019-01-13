import { Form, Input, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized } from '../../components/authorized'
import { formItemLayout } from '../../components/form'
import Layout from '../../components/form/Layout'
import Submit from '../../components/form/Submit'
import { httpPost } from '../../utils/request'

const FormItem = Form.Item

class Widget extends React.Component<InjectedIntlProps & FormComponentProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public comparePasswords = (_: any, value: string, callback: (m?: string) => void) => {
    const { form, intl } = this.props
    if (value && value !== form.getFieldValue('newPassword')) {
      callback(intl.formatMessage({ id: "form.validations.password-confirmation" }))
    } else {
      callback()
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        httpPost("/users/change-password", values).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
          form.setFieldsValue({ currentPassword: '', newPassword: '', passwordConfirmation: '' })
        }).catch(message.error)
      }
    })
  }
  public render() {
    const { formatMessage } = this.props.intl
    const { getFieldDecorator } = this.props.form

    return (<Authorized>
      <Layout title={{ id: "nut.users.change-password.title" }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.current-password" />}>
            {
              getFieldDecorator('currentPassword', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input type="password" />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.new-password" />}>
            {
              getFieldDecorator('newPassword', {
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
      </Layout>
    </Authorized>)
  }
}


export default injectIntl(Form.create()(Widget))
