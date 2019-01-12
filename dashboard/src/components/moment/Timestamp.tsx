import * as React from 'react'
import { InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import Moment from 'react-moment'

import { detect, timezone } from '.'

interface IProps {
  date: Date,
}

class Widget extends React.Component<InjectedIntlProps & IProps> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  public render() {
    return (<Moment
      locale={detect(this.props.intl.locale)}
      utc={true}
      tz={timezone}
      format="LLLL"
      date={this.props.date} />)
  }
}

export default injectIntl(Widget)
