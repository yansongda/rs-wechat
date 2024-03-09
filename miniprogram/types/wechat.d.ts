import { Response } from './http'

export interface WxRequestSuccess<T> {
  data: Response<T>
  statusCode: Number
  header: Object
}

export interface WxRequestFail {
  errno: Number
  errMsg: String
}

export interface AppOnUnhandledRejection {
  /** 被拒绝的 Promise 对象 */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  promise: Promise<any>
  /** 拒绝原因，一般是一个 Error 对象 */
  reason: Error | String
}

export interface Tap<T, D> {
  type: String
  currentTarget: {
    id: String
    dataset: T
  }
  target: {
    id: String
    dataset: D
  }
}

export interface FormSubmit<T> {
  detail: {
    formId: String
    value: T
  }
}

export interface ChooseAvatarButtonTap<T, D> extends Tap<T, D> {
  detail: {
    avatarUrl: String
  }
}

export interface WxGetFileSystemManagerReadFileSuccess {
  data: String | ArrayBuffer
}

export interface WxGetUpdateManagerOnCheckForUpdateResult {
  hasUpdate: Boolean
}
