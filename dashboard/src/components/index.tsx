import { MessageBarType } from 'office-ui-fabric-react/lib/MessageBar'

export interface ILabel {
    id: string,
    value?: object,
}

export interface IMessageBar {
    content: string,
    type: MessageBarType,
}

