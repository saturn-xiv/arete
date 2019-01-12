import * as moment from 'moment-timezone'

export const timezone = moment.tz.guess()

export const detect = (l: string): string => {
  switch (l) {
    case 'zh-Hans':
      return 'zh-cn'
    case 'zh-Hant':
      return 'zh-tw'
    default:
      return 'en'
  }
}
