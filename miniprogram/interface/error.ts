interface IERROR {
  code: number,
  message: string,
}

interface ILoginError extends IERROR {
  describe?: string,
}

interface IHttpError extends IERROR {
  describe?: string,
  url?: string,
  headers?: IRequestHeaders,
  timeout?: number,
}

interface IHttpApiError extends IERROR {
  query?: IRequestQuery,
  json?: IRequestJson,
  data?: IRequestData,
  headers?: IRequestHeaders,
}
