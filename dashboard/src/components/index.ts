import { MessageValue } from 'react-intl'

export interface ILabel {
  id: string,
  values?: { [key: string]: MessageValue },
}
