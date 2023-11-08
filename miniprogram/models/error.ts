import { CODE, MESSAGE } from '@constant/error'

class EError extends Error implements IERROR {
  code: number

  constructor(code?: number, message?: string) {
    super()

    this.code = code || CODE.UNKNOWN
    this.message = message || MESSAGE[this.code] || MESSAGE[CODE.UNKNOWN]
  }
}

class WeixinError extends EError {
  constructor(code?: number, message?: string) {
    super(code || CODE.WEIXIN, message)
  }
}

class LoginError extends EError implements ILoginError {
  constructor(code?: number, message?: string) {
    super(code || CODE.LOGIN, message)
  }
}

class HttpError extends EError implements IHttpError {
  url?: string
  headers?: IRequestHeaders
  query?: IRequestQuery
  data?: IRequestData
  timeout?: number

  constructor(code?: number, message?: string) {
    super(code || CODE.HTTP, message)
  }
}

export { EError, WeixinError, LoginError, HttpError }
