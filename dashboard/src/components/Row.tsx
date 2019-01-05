import * as React from 'react'

class Widget extends React.Component {
    public render() {
        return (<div className="ms-Grid-row">{this.props.children}</div>)
    }
}

export default Widget