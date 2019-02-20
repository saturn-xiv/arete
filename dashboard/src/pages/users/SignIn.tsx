import { Form, Input, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'
import { Dispatch } from 'redux'

import { ISiteState, siteRefresh, userSignIn } from '../../actions'
import { formItemLayout } from '../../components/form'
import Submit from '../../components/form/Submit'
import { IApplicationState } from '../../reducers'
import { httpPost } from '../../utils/request'
import Layout from './SharedLinks'

interface IProps {
  signIn: typeof userSignIn,
  site: ISiteState,
  refresh: typeof siteRefresh,
}

const FormItem = Form.Item

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, signIn, site, refresh, history } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        httpPost("/users/sign-in", values).then((rst) => {
          signIn(rst.token)
          refresh(Object.assign({}, site, { who: rst.info }))
          history.push("/users/logs")
        }).catch(message.error)
      }
    })
  }
  public render() {
    const { formatMessage } = this.props.intl
    const { getFieldDecorator } = this.props.form

    return (<Layout title="nut.users.sign-in.title">
      <Form onSubmit={this.handleSubmit}>
        <FormItem {...formItemLayout} label={<FormattedMessage id="nut.users.sign-in.login" />}>
          {
            getFieldDecorator('login', {
              rules: [
                {
                  message: formatMessage({ id: "form.validations.required" }),
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
                  message: formatMessage({ id: "form.validations.required" }),
                  required: true,
                }
              ]
            })(<Input type="password" />)
          }
        </FormItem>
        <Submit />
      </Form>
    </Layout>)
  }
}

const mapStateToProps = ({ site }: IApplicationState) => ({
  site
})

const mapDispatchToProps = (dispatch: Dispatch) => ({
  refresh: (info: ISiteState) => dispatch(siteRefresh(info)),
  signIn: (token: string) => dispatch(userSignIn(token)),
})

export default withRouter(connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(Form.create()(Widget))))
