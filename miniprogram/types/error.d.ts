import { RequestData, RequestHeaders, RequestQuery } from './http'

export interface ERROR {
  code: number
  message: string
}

export interface LoginError extends ERROR {}

export interface HttpError extends ERROR {
  url?: string
  headers?: RequestHeaders
  query?: RequestQuery
  data?: RequestData
  timeout?: number
}
