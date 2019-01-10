import { Icon } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'
import { connect } from 'react-redux'

import { ISiteState } from '../actions'
import { IApplicationState } from '../reducers'

interface IProps {
  site: ISiteState,
}

class Widget extends React.Component<IProps> {
  public render() {
    return (<div>
      <Icon type="copyright" />
      &nbsp;
            <FormattedMessage id="site.copyright" />
      &nbsp;
            {this.props.site.version}
    </div>)
  }
}


const mapStateToProps = ({ site }: IApplicationState) => ({
  site,
})


const mapDispatchToProps = () => ({})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Widget)
