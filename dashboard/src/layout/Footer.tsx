import GlobalFooter from 'ant-design-pro/lib/GlobalFooter'
import { Icon } from 'antd'
import * as React from 'react'

import Copyright from './Copyright'

interface ILink {
    icon: string,
    to: string,
}

interface IState {
    links: ILink[],
}

class Widget extends React.Component<{}, IState> {
    constructor(props: any) {
        super(props)
        this.state = {
            links: [
                { icon: 'home', to: '/' },
                { icon: 'github', to: 'https://github.com/saturn-xiv/arete' },
            ]
        }
    }
    public render() {
        return (<GlobalFooter links={this.state.links.map((it) => {
            return {
                blankTarget: true,
                href: it.to,
                key: it.to,
                title: (<Icon type={it.icon} />),
            }
        })} copyright={<Copyright />} />)
    }
}

export default Widget