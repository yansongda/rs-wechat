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
  headers?: IHeaders,
  timeout?: number,
}

interface IHttpApiError extends IERROR {
  query?: IQuery,
  json?: IJson,
  data?: IData,
  headers?: IHeaders,
}
