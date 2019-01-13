import { Form, Input, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized } from '../../components/authorized'
import { formItemLayout } from '../../components/form'
import Layout from '../../components/form/Layout'
import Submit from '../../components/form/Submit'
import { httpGet, httpPost } from '../../utils/request'

const FormItem = Form.Item

class Widget extends React.Component<InjectedIntlProps & FormComponentProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public componentDidMount() {
    const { form } = this.props
    httpGet(`/users/profile`).then((rst) => {
      form.setFieldsValue(rst)
    }).catch(message.error)
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        httpPost("/users/profile", values).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
        }).catch(message.error)
      }
    })
  }
  public render() {
    const { formatMessage } = this.props.intl
    const { getFieldDecorator } = this.props.form

    return (<Authorized>
      <Layout title={{ id: "nut.users.profile.title" }}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.nick-name" />}>
            {
              getFieldDecorator('nickName', {
                rules: [
                  {
                    required: true,
                  }
                ]
              })(<Input disabled={true} />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.email" />}>
            {
              getFieldDecorator('email', {
                rules: [
                  {
                    required: true,
                  }
                ]
              })(<Input disabled={true} type="email" />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.real-name" />}>
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
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.logo" />}>
            {
              getFieldDecorator('logo', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>

          <Submit />
        </Form>
      </Layout>
    </Authorized>)
  }
}


export default injectIntl(Form.create()(Widget))
