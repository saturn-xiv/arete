import { Form, Input, message, Select } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'

import { ILabel } from '../../../components'
import { Authorized, RoleTypes } from '../../../components/authorized'
import { formItemLayout } from '../../../components/form'
import Layout from '../../../components/form/Layout'
import Submit from '../../../components/form/Submit'
import { httpGet, httpPost } from '../../../utils/request'

const Option = Select.Option
const FormItem = Form.Item

interface IState {
  title: ILabel,
}

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps, IState> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  constructor(props: RouteComponentProps<any> & InjectedIntlProps & FormComponentProps) {
    super(props)
    this.state = {
      title: { id: 'nut.admin.friend-links.new.title' }
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl, history, match } = this.props
    const id = match.params.id
    form.validateFields((err, values) => {
      if (!err) {
        httpPost(
          (id ? `/admin/friend-links/${id}` : '/admin/friend-links'),
          values,
        ).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
          history.push('/admin/friend-links')
        }).catch(message.error)
      }
    })
  }

  public componentDidMount() {
    const { form, match } = this.props
    const id = match.params.id
    if (id) {
      httpGet(`/admin/friend-links/${id}`).then((rst) => {
        form.setFieldsValue({
          home: rst.home,
          logo: rst.logo,
          position: rst.position,
          title: rst.title,
        })
        this.setState({
          title: {
            id: "nut.admin.friend-links.edit.title",
            values: {
              title: rst.title,
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
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.title" />}>
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
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.models.friend-link.home" />}>
            {
              getFieldDecorator('home', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
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
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.position" />}>
            {
              getFieldDecorator('position', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Select style={{ width: 150 }} >
                {Array.from({ length: 20 }, (v, i) => i - 10).map((it) => (<Option key={it.toString()} value={it}>{it}</Option>))}
              </Select>)
            }
          </FormItem>

          <Submit />
        </Form>
      </Layout>
    </Authorized >)
  }
}


const mapStateToProps = () => ({})


const mapDispatchToProps = () => ({})

export default withRouter(connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(Form.create()(Widget))))
