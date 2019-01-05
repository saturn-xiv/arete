import { INavLink, Nav } from 'office-ui-fabric-react/lib/Nav'
import * as React from 'react'
import { InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { RouteComponentProps, withRouter } from "react-router"


import Col from './Col'
import Row from './Row'

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps> {
    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired,
    }

    constructor(props: RouteComponentProps<any> & InjectedIntlProps) {
        super(props)
        this.handleClickItem = this.handleClickItem.bind(this)
    }

    public handleClickItem(ev: React.MouseEvent<HTMLElement>, it: INavLink) {
        ev.preventDefault();
        this.props.history.push(it.url)
    }

    public render() {
        const { formatMessage } = this.props.intl
        return (<Row>
            <Col width={{ sm: 10, md: 4 }} offset={{ sm: 1, md: 4 }}>
                {this.props.children}
                <Nav
                    onLinkClick={this.handleClickItem}
                    groups={[
                        {
                            links: [
                                { icon: 'Assign', label: 'users.sign-in.title', to: '/users/sign-in' },
                                { icon: 'PeopleAdd', label: 'users.sign-up.title', to: '/users/sign-up' },
                                { icon: 'LaptopSecure', label: 'users.forgot-password.title', to: '/users/forgot-password' },
                                { icon: 'ActivateOrders', label: 'users.confirm.title', to: '/users/confirm' },
                                { icon: 'Unlock', label: 'users.unlock.title', to: '/users/unlock' },
                                { icon: 'Message', label: 'leave-words.new.title', to: '/leave-words/new' },
                            ].map((it) => {
                                return { icon: it.icon, name: formatMessage({ id: it.label }), key: it.to, url: it.to }
                            })
                        }
                    ]}
                />
            </Col>
        </Row>)
    }
}

export default injectIntl(withRouter(Widget))