import * as React from 'react'

import { MediaType } from '.'

interface IProps {
  body: string,
  mediaType: MediaType,
}

class Widget extends React.Component<IProps> {
  public render() {
    switch (this.props.mediaType) {
      case MediaType.TEXT:
        return (<pre>{this.props.body}</pre>)
      default:
        return (<>TODO</>)
    }
  }
}

export default Widget
