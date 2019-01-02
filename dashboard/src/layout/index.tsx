import * as React from 'react'

import Footer from './Footer'
import Header from './Header'

interface IWidgetProps {
    children: JSX.Element | JSX.Element[];
}

class Widget extends React.Component<IWidgetProps> {
    public render() {
        return (<div>
            <Header />
            <div className="ms-Grid" dir="ltr">
                {this.props.children}
            </div>
            <Footer />
        </div>)
    }
}

export default Widget