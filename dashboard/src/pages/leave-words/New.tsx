import { Form, Input, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { MediaType } from '../../components'
import { formItemLayout } from '../../components/form'
import Submit from '../../components/form/Submit'
import { httpPost } from '../../utils/request'
import Layout from '../users/SharedLinks'

const FormItem = Form.Item

class Widget extends React.Component<InjectedIntlProps & FormComponentProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        httpPost("/leave-words", Object.assign({}, values, { mediaType: MediaType.TEXT })).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
          form.setFieldsValue({ body: '' })
        }).catch(message.error)
      }
    })
  }
  public render() {
    const { formatMessage } = this.props.intl
    const { getFieldDecorator } = this.props.form

    return (<Layout title="nut.leave-words.new.title">
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.body" />}>
          {
            getFieldDecorator('body', {
              rules: [
                {
                  message: formatMessage({ id: "form.validations.required" }),
                  required: true,
                },
              ]
            })(<Input.TextArea rows={6} />)
          }
        </FormItem>

        <Submit />
      </Form>
    </Layout>)
  }
}


export default injectIntl(Form.create()(Widget))
