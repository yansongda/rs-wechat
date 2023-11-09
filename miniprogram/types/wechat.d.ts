import { Response } from './http'

export interface WxRequestSuccess<T> {
  data: Response<T>
  statusCode: number
  header: object
}

export interface WxRequestFail {
  errno: number
  errMsg: string
}

export interface AppOnUnhandledRejection {
  /** 被拒绝的 Promise 对象 */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  promise: Promise<any>
  /** 拒绝原因，一般是一个 Error 对象 */
  reason: Error | string
}
