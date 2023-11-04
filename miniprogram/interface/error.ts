interface IERROR {
  code: number,
  message: string,
}

interface ILoginError extends IERROR {
}

interface IHttpError extends IERROR {
  url?: string,
  headers?: IRequestHeaders,
  query?: IRequestQuery,
  data?: IRequestData,
  timeout?: number,
}
