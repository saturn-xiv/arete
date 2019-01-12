import Exception from 'ant-design-pro/lib/Exception'
import * as React from 'react'
import { connect } from 'react-redux'


import { IUserState } from '../actions'
import { IApplicationState } from '../reducers'

export const ADMIN = 'admin'
export const ROOT = 'root'

interface IProps {
  user: IUserState,
  authority?: string,
  children: React.ReactNode,
}

class Widget extends React.Component<IProps> {

  public render() {
    return this.havePermission() ? this.props.children : (<Exception type="403" />)
  }

  private havePermission(): boolean {
    const { user, authority } = this.props
    if (user.uid) {
      if (authority) {
        if (user.roles.indexOf(ROOT) >= 0) {
          return true
        }
        if (authority !== ROOT && user.roles.indexOf(ADMIN) >= 0) {
          return true
        }
        return user.roles.indexOf(authority) >= 0
      } else {
        return true
      }
    }
    return false
  }

}

const mapStateToProps = ({ user }: IApplicationState) => ({ user })

const mapDispatchToProps = () => ({})

export const Authorized = connect(
  mapStateToProps,
  mapDispatchToProps
)(Widget)
