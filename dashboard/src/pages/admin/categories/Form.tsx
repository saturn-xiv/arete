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
import ColorPicker from '../../../components/form/ColorPicker'
import Layout from '../../../components/form/Layout'
import Submit from '../../../components/form/Submit'
import availableIcons from '../../../icons'
import { IApplicationState } from '../../../reducers'
import { httpGet, httpPost } from '../../../utils/request'
import { IItem } from './Index'

const Option = Select.Option
const FormItem = Form.Item

interface IProps {
  site: ISiteState,
}

interface IState {
  color: string,
  title: ILabel,
  items: IItem[],
}

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps, IState> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  constructor(props: RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps) {
    super(props)
    this.state = {
      color: '#000000',
      items: [],
      title: { id: 'nut.admin.categories.new.title' }
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl, history, match } = this.props
    const id = match.params.id
    form.validateFields((err, values) => {
      if (!err) {
        httpPost(
          (id ? `/admin/categories/${id}` : '/admin/categories'),
          Object.assign({}, values, { color: this.state.color }),
        ).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
          history.push('/admin/categories')
        }).catch(message.error)
      }
    })
  }
  public handleColorChange = (v: string) => {
    this.setState({ color: v })
  }

  public componentDidMount() {
    const { form, match } = this.props
    const id = match.params.id
    httpGet(`/admin/categories`).then((rst) => {
      this.setState({ items: id ? rst.filter((it: IItem) => it.id !== parseInt(id, 10)) : rst })
    }).catch(message.error)

    if (id) {
      httpGet(`/admin/categories/${id}`).then((rst) => {
        form.setFieldsValue({ name: rst.name, icon: rst.icon, position: rst.position })
        this.setState({
          color: rst.color,
          title: {
            id: "nut.admin.categories.edit.title",
            values: {
              name: rst.name,
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
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.name" />}>
            {
              getFieldDecorator('name', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.parent" />}>
            {
              getFieldDecorator('parent', { rules: [] })(<Select allowClear={true} style={{ width: 320 }} >
                {this.state.items.map((it) => (<Option key={it.id.toString()} value={it.id}>{it.name}</Option>))}
              </Select>)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.icon" />}>
            {
              getFieldDecorator('icon', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Select style={{ width: 150 }} >
                {availableIcons.map((it) => (<Option key={it} value={it}>{it}</Option>))}
              </Select>)
            }
          </FormItem>
          <ColorPicker defaultValue={this.state.color} onChange={this.handleColorChange} />
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


const mapStateToProps = ({ site }: IApplicationState) => ({
  site
})


const mapDispatchToProps = () => ({
})

export default withRouter(connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(Form.create()(Widget))))
