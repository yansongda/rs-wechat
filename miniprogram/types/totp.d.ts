import { RequestData } from './http'

export interface Item {
  id: number
  issuer: string
  username: string
  code: string
}

export interface Interval {

}

export interface IntervalItem {
  id: number
  interval: number
  identity: number
}

export interface DetailRequest extends RequestData {
  id: number
}

export interface UpdateRequest extends RequestData {
  id: number
  issuer: string
  username: string
}

export interface CreateRequest extends RequestData {
  uri: string
}

export interface DeleteRequest extends RequestData {
  id: number
}

export interface Response {}
