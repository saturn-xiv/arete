import { PrimaryButton } from 'office-ui-fabric-react/lib/Button'
import * as React from 'react'

import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

interface ITitle {
    id: string,
    value?: object,
}

interface IWidgetProps {
    children: JSX.Element | JSX.Element[],
    title: ITitle,
}

class Widget extends React.Component<IWidgetProps & InjectedIntlProps> {
    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired
    };
    public render() {
        return (<div className="ms-Grid-row">
            <div className="ms-Grid-col ms-sm10 ms-smPush1 ms-md4 ms-mdPush4">
                <FormattedMessage {...this.props.title} tagName="h2" />
                {this.props.children}
                <br />
                <PrimaryButton text={this.props.intl.formatMessage({ id: 'buttons.submit' })} />
            </div>
        </div>)
    }
}

export default injectIntl(Widget)