import { Form, Input, message, Select } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'

import { ISiteState } from '../../../actions'
import { ILabel } from '../../../components'
import { Authorized, RoleTypes } from '../../../components/authorized'
import { formItemLayout, TEXTAREA_ROWS } from '../../../components/form'
import Layout from '../../../components/form/Layout'
import Submit from '../../../components/form/Submit'
import { IApplicationState } from '../../../reducers'
import { httpGet, httpPost } from '../../../utils/request'

const FormItem = Form.Item
const Option = Select.Option

interface IProps {
  site: ISiteState,
}

interface IState {
  title: ILabel,
}

class Widget extends React.Component<InjectedIntlProps & FormComponentProps & IProps, IState> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  constructor(props: InjectedIntlProps & FormComponentProps & IProps) {
    super(props)
    this.state = {
      title: { id: 'nut.admin.site.info.title' }
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        httpPost(`/admin/site/info/${values.lang}`, values).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
        }).catch(message.error)
      }
    })
  }
  public handleLangChange = (lang: string) => {
    const { form } = this.props
    httpGet(`/admin/site/info/${lang}`).then((rst) => {
      form.setFieldsValue(rst)
    }).catch((e) => {
      message.error(e)
      form.setFieldsValue({
        copyright: '',
        description: '',
        keywords: '',
        subhead: '',
        title: '',
      })
    })
  }
  public componentDidMount() {
    const { form } = this.props
    const lang = this.props.intl.locale
    form.setFieldsValue({ lang })
    this.handleLangChange(lang)
  }
  public render() {
    const { formatMessage } = this.props.intl
    const { getFieldDecorator } = this.props.form

    return (<Authorized authority={RoleTypes.ADMIN}>
      <Layout title={this.state.title}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.locale.lang" />}>
            {
              getFieldDecorator('lang', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Select onChange={this.handleLangChange} style={{ width: 150 }} >
                {this.props.site.languages.map((it) => (<Option key={it} value={it}><FormattedMessage id={`languages.${it}`} /></Option>))}
              </Select>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.site.title" />}>
            {
              getFieldDecorator('title', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.site.subhead" />}>
            {
              getFieldDecorator('subhead', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.site.keywords" />}>
            {
              getFieldDecorator('keywords', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.site.description" />}>
            {
              getFieldDecorator('description', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  },
                ]
              })(<Input.TextArea rows={TEXTAREA_ROWS} />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.site.copyright" />}>
            {
              getFieldDecorator('copyright', {
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
    </Authorized >)
  }
}


const mapStateToProps = ({ site }: IApplicationState) => ({
  site
})


const mapDispatchToProps = () => ({
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(Form.create()(Widget)))
