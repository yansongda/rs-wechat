import { RequestData } from './http'

export interface CreateRequest extends RequestData {
  link: string
}

export interface CreateResponse {
  link: string
  short: string
}
