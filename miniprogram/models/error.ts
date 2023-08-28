import { CODE, MESSAGE } from '@constant/error'

class EError extends Error implements IERROR {
  code: number;

  constructor(code?: number, message?: string) {
    super()

    this.code = code || CODE.UNKNOWN
    this.message = message || MESSAGE[this.code]
  }
}

class WeixinError extends EError {}

class LoginError extends EError implements ILoginError {
  describe?: string;

  constructor(describe?: string, code?: number, message?: string) {
    super(code || CODE.LOGIN, message)

    this.describe = describe
  }
}

class HttpError extends EError implements IHttpError {
  describe?: string;
  url?: string;
  headers?: IRequestHeaders;
  timeout?: number;

  constructor(describe?: string, code?: number, message?: string) {
    super(code || CODE.HTTP, message)

    this.describe = describe
  }
}

class HttpApiError extends EError implements IHttpApiError {
  query?: IRequestQuery;
  json?: IRequestJson;
  data?: IRequestData;
  headers?: IRequestHeaders;

  constructor(code?: number, message?: string) {
    super(code || CODE.HTTP, message)
  }
}

export { EError, WeixinError, LoginError, HttpError, HttpApiError }