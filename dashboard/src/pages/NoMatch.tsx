import Exception from 'ant-design-pro/lib/Exception'
import * as React from 'react'

import Head from '../components/Head'

class Widget extends React.Component {
  public render() {
    return (<>
      <Exception type="404" />
      <Head title={{ id: "flashes.not-found" }} />
    </>)
  }
}

export default Widget
