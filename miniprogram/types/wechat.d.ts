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

export interface Tap<T, D> {
  type: string
  currentTarget: {
    id: string
    dataset: T
  }
  target: {
    id: string
    dataset: D
  }
}

export interface FormSubmit<T> {
  detail: {
    formId: string
    value: T
  }
}

export interface WeuiDialogTap {
  detail: {
    index: number
    item: {
      text?: string
      extClass?: string
    }
  }
}

export interface WeuiSlideviewButtonTap<T, D> extends Tap<T, D> {
  detail: {
    index: number
  }
}

export interface ChooseAvatarButtonTap<T, D> extends Tap<T, D> {
  detail: {
    avatarUrl: string
  }
}

export interface WxGetFileSystemManagerReadFileSuccess {
  data: string | ArrayBuffer
}

export interface WxGetUpdateManagerOnCheckForUpdateResult {
  hasUpdate: boolean
}
