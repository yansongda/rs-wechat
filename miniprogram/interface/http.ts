interface IRequest {
  url: string,
  query?: IRequestQuery,
  data?: IRequestData,
  headers?: IRequestHeaders,
  method?: "POST" | "OPTIONS" | "GET" | "HEAD" | "PUT" | "DELETE" | "TRACE" | "CONNECT" | undefined,
  timeout?: number,
  isUploadFile?: boolean,
}

interface IRequestQuery {
  [key: string]: number | string | undefined
}

interface IRequestData {
  [key: string]: any
}

interface IRequestHeaders {
  [key: string]: string | undefined
}
