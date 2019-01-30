import { Form, Input, message, Select } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'

import { ISiteState } from '../../../actions'
import { ILabel, MediaType } from '../../../components'
import { Authorized, RoleTypes } from '../../../components/authorized'
import { formItemLayout, LANGUAGE_WIDTH } from '../../../components/form'
import Layout from '../../../components/form/Layout'
import Quill from '../../../components/form/Quill'
import Submit from '../../../components/form/Submit'
import { IApplicationState } from '../../../reducers'
import { httpGet, httpPost } from '../../../utils/request'

const Option = Select.Option
const FormItem = Form.Item

interface IProps {
  site: ISiteState,
}

interface IState {
  body: string,
  title: ILabel,
}

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps, IState> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  constructor(props: RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps) {
    super(props)
    this.state = {
      body: '',
      title: { id: 'nut.admin.cards.new.title' }
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl, history, match } = this.props
    const id = match.params.id
    form.validateFields((err, values) => {
      if (!err) {
        httpPost(
          (id ? `/admin/cards/${id}` : '/admin/cards'),
          Object.assign({}, values, { body: this.state.body, mediaType: MediaType.HTML }),
        ).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
          history.push('/admin/cards')
        }).catch(message.error)
      }
    })
  }

  public handleBodyChange = (v: string) => {
    this.setState({ body: v })
  }

  public componentDidMount() {
    const { form, match } = this.props
    const id = match.params.id
    if (id) {
      httpGet(`/admin/cards/${id}`).then((rst) => {
        form.setFieldsValue({
          action: rst.action,
          href: rst.href,
          lang: rst.lang,
          loc: rst.loc,
          logo: rst.logo,
          position: rst.position,
          title: rst.title,
        })
        this.setState({
          body: rst.body,
          title: {
            id: "nut.admin.cards.edit.title",
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
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.lang" />}>
            {
              getFieldDecorator('lang', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Select style={{ width: LANGUAGE_WIDTH }} >
                {this.props.site.languages.map((it) => (<Option key={it} value={it}><FormattedMessage id={`languages.${it}`} /></Option>))}
              </Select>)
            }
          </FormItem>
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
          <Quill value={this.state.body} onChange={this.handleBodyChange} />
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
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.href" />}>
            {
              getFieldDecorator('href', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="buttons.action" />}>
            {
              getFieldDecorator('action', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.loc" />}>
            {
              getFieldDecorator('loc', {
                rules: [
                  {
                    message: formatMessage({ id: "form.validations.required" }),
                    required: true,
                  }
                ]
              })(<Select style={{ width: 150 }} >
                {['bootstrap-carousel', 'bootstrap-center', 'bootstrap-bottom'].map((it) => (<Option key={it} value={it}>{it}</Option>))}
              </Select>)
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


const mapStateToProps = ({ site }: IApplicationState) => ({
  site
})


const mapDispatchToProps = () => ({
})

export default withRouter(connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(Form.create()(Widget))))
