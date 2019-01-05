import * as React from 'react'

interface IWidth {
    md: number,
    sm: number,
}
interface IOffset {
    md: number,
    sm: number,
}

interface IProps {
    width: IWidth,
    offset?: IOffset,
}

class Widget extends React.Component<IProps> {
    public render() {
        let cn = `ms-Grid-col ms-sm${this.props.width.sm} ms-md${this.props.width.md}`
        if (this.props.offset) {
            cn += ` ms-smPush${this.props.offset.sm}  ms-mdPush${this.props.offset.md}`
        }
        return (<div className={cn}>{this.props.children}</div>)
    }
}

export default Widget