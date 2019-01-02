import * as React from 'react'

import Footer from './Footer'
import Header from './Header'

class Widget extends React.Component<any, any> {
    public render() {
        return (<div>
            <Header />
            {this.props.children}
            <Footer />
        </div>)
    }
}

export default Widget