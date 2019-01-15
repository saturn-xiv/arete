import { Form, Input, List, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'

import { ISiteState } from '../../../actions'
import { ILabel } from '../../../components'
import { Authorized, RoleTypes } from '../../../components/authorized'
import { formItemLayout } from '../../../components/form'
import Layout from '../../../components/form/Layout'
import Submit from '../../../components/form/Submit'
import { IApplicationState } from '../../../reducers'
import { httpGet, httpPost } from '../../../utils/request'

const FormItem = Form.Item

interface IProps {
  site: ISiteState,
}

interface IGoogle {
  siteVerifyId: string,
}
interface IBaidu {
  siteVerifyId: string,
}
interface IState {
  title: ILabel,
  google: IGoogle,
  baidu: IBaidu,
}

class Widget extends React.Component<InjectedIntlProps & FormComponentProps & IProps, IState> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  constructor(props: InjectedIntlProps & FormComponentProps & IProps) {
    super(props)
    this.state = {
      baidu: { siteVerifyId: '' },
      google: { siteVerifyId: '' },
      title: { id: 'nut.admin.site.seo.title' }
    }
  }
  public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const { form, intl } = this.props
    form.validateFields((err, values) => {
      if (!err) {
        httpPost(`/admin/site/seo`, {
          baidu: { siteVerifyId: values.baiduSiteVerifyId },
          google: { siteVerifyId: values.googleSiteVerifyId },
        }).then((_) => {
          message.success(intl.formatMessage({ id: "flashes.success" }))
        }).catch(message.error)
      }
    })
  }
  public componentDidMount() {
    const { form } = this.props
    httpGet(`/admin/site/seo`).then((rst) => {
      if (rst.google) {
        form.setFieldsValue({ googleSiteVerifyId: rst.google.siteVerifyId })
      }
      if (rst.baidu) {
        form.setFieldsValue({ baiduSiteVerifyId: rst.baidu.siteVerifyId })
      }

      this.setState(rst)
    }).catch(message.error)
  }

  public render() {
    const { getFieldDecorator } = this.props.form

    return (<Authorized authority={RoleTypes.ADMIN}>
      <Layout title={this.state.title}>
        <Form onSubmit={this.handleSubmit}>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.admin.site.seo.google.site-verify-id" />}>
            {
              getFieldDecorator('googleSiteVerifyId', { rules: [] })(<Input />)
            }
          </FormItem>
          <FormItem {...formItemLayout} label={<FormattedMessage id="nut.admin.site.seo.baidu.site-verify-id" />}>
            {
              getFieldDecorator('baiduSiteVerifyId', { rules: [] })(<Input />)
            }
          </FormItem>

          <Submit />
        </Form>
        <List
          size="small"
          bordered={true}
          dataSource={this.props.site.languages.map((it) => `/rss/${it}`).concat([`/google${this.state.google.siteVerifyId}.html`, `/baidu_verify_${this.state.baidu.siteVerifyId}.html`, '/robots.txt', '/sitemap.xml.gz'])}
          renderItem={(it: string) => (<List.Item><a href={it} target="_blank">{it}</a></List.Item>)}
        />
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
