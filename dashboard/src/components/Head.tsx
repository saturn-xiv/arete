import * as React from 'react'
import { Helmet } from "react-helmet"
import { InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { ILabel } from '.'

interface IProps {
  title: ILabel,
}

class Widget extends React.Component<InjectedIntlProps & IProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public render() {
    const { formatMessage } = this.props.intl
    const title = this.props.title
    return (<Helmet>
      <title>{formatMessage({ id: title.id }, title.values)} | {formatMessage({ id: 'site.subhead' })} | {formatMessage({ id: 'site.title' })}</title>
    </Helmet>)
  }
}

export default injectIntl(Widget)
