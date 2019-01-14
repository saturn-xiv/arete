import { Form, Input, message, Select } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'

import { ISiteState } from '../../../actions'
import { ILabel } from '../../../components'
import { Authorized, RoleTypes } from '../../../components/authorized'
import { formItemLayout } from '../../../components/form'
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

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps, IState> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  constructor(props: RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps) {
    super(props)
    this.state = {
      title: { id: 'nut.admin.locales.new.title' }
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl, history, match } = this.props
    const id = match.params.id
    form.validateFields((err, values) => {
      if (!err) {
        httpPost((id ? `/admin/locales/${id}` : '/admin/locales'), values).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
          history.push('/admin/locales')
        }).catch(message.error)
      }
    })
  }
  public componentDidMount() {
    const { form, match } = this.props
    const id = match.params.id
    if (id) {
      httpGet(`/admin/locales/${id}`).then((rst) => {
        form.setFieldsValue({ lang: rst.lang, code: rst.code, message: rst.message })
        this.setState({
          title: {
            id: "nut.admin.locales.edit.title",
            values: {
              code: rst.code,
              lang: rst.lang,
            },
          }
        })
      }).catch(message.error)
    }
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
              })(<Select style={{ width: 150 }} >
                {this.props.site.languages.map((it) => (<Option key={it} value={it}><FormattedMessage id={`languages.${it}`} /></Option>))}
              </Select>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.locale.code" />}>
            {
              getFieldDecorator('code', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.locale.message" />}>
            {
              getFieldDecorator('message', {
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
      </Layout>
    </Authorized >)
  }
}


const mapStateToProps = ({ site }: IApplicationState) => ({
  site
})


const mapDispatchToProps = () => ({
})

export default withRouter(connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(Form.create()(Widget))))
