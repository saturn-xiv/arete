import * as React from 'react'
import { Helmet } from "react-helmet"
import { InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { ILabel } from '../components'

interface IProps {
  title: ILabel,
}

class Widget extends React.Component<InjectedIntlProps & IProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public render() {
    const { formatMessage } = this.props.intl
    return (<Helmet>
      <title>{formatMessage(this.props.title)} | {formatMessage({ id: 'site.subhead' })} | {formatMessage({ id: 'site.title' })}</title>
    </Helmet>)
  }
}

export default injectIntl(Widget)
