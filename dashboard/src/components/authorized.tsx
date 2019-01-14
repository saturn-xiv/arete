import Exception from 'ant-design-pro/lib/Exception'
import * as React from 'react'
import { connect } from 'react-redux'

import { IUserState } from '../actions'
import { IApplicationState } from '../reducers'
import Head from './Head'

export const enum RoleTypes {
  ROOT = 'root',
  ADMIN = 'admin',
}

export const havePermission = (user: IUserState, authority?: string): boolean => {
  if (user.uid) {
    if (authority) {
      if (user.roles.indexOf(RoleTypes.ROOT) >= 0) {
        return true
      }
      if (authority !== RoleTypes.ROOT && user.roles.indexOf(RoleTypes.ADMIN) >= 0) {
        return true
      }
      return user.roles.indexOf(authority) >= 0
    } else {
      return true
    }
  }
  return false
}

interface IProps {
  user: IUserState,
  authority?: string,
  children: React.ReactNode,
}

class Widget extends React.Component<IProps> {
  public render() {
    const { user, authority } = this.props
    return havePermission(user, authority) ? this.props.children : (<><Exception type="403" /><Head title={{ id: "flashes.forbidden" }} /></>)
  }
}

const mapStateToProps = ({ user }: IApplicationState) => ({ user })

const mapDispatchToProps = () => ({})

export const Authorized = connect(
  mapStateToProps,
  mapDispatchToProps
)(Widget)
