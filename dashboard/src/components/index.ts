import { MessageValue } from 'react-intl'

export const enum MediaType {
  HTML = 'html',
  TEXT = 'text',
  MARKDOWN = 'markdown',
}

export interface ILabel {
  id: string,
  values?: { [key: string]: MessageValue },
}
