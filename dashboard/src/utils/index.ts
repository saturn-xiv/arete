import * as URI from 'urijs'

export const HOME = URI(window.location).origin()

export const url = (p: string) => URI(p).absoluteTo(window.location.toString()).toString()
