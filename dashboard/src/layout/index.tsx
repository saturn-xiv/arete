import { message } from 'antd'
import * as React from 'react'
import { connect } from 'react-redux'
import { Dispatch } from 'redux'

import { ISiteState, IUserState, refresh as refreshSiteInfo } from '../actions'
import { IApplicationState } from '../reducers'
import { httpGet } from '../utils/request'
import Footer from './Footer'

interface IProps {
    children: React.ReactNode,
    user: IUserState,
    site: ISiteState,
    refresh: typeof refreshSiteInfo,
}

interface IState {
    collapsed: boolean,
}

class Widget extends React.Component<IProps, IState> {
    constructor(props: IProps) {
        super(props)
        this.state = {
            collapsed: false,
        }
    }
    public componentDidMount() {
        httpGet(`/about`).then((rst) => {
            this.props.refresh(rst)
        }).catch(message.error)
    }
    public render() {
        return (<div>
            {this.props.children}
            <Footer />
        </div>)
    }
}

const mapStateToProps = ({ site, user }: IApplicationState) => ({
    site,
    user,
})


const mapDispatchToProps = (dispatch: Dispatch) => ({
    refresh: (info: ISiteState) => dispatch(refreshSiteInfo(info))
})

export default connect(
    mapStateToProps,
    mapDispatchToProps
)(Widget)
