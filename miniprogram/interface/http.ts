interface IOptions {
  query?: IQuery,
  json?: IJson,
  data?: IData,
  headers?: IHeaders,
  method?: "POST" | "OPTIONS" | "GET" | "HEAD" | "PUT" | "DELETE" | "TRACE" | "CONNECT" | undefined,
  timeout?: number,
}

interface IQuery{
  [key: string]: number | string | undefined
}

interface IJson{
  [key: string]: number | string | null | undefined
}

interface IData{
  [key: string]: any
}

interface IHeaders{
  [key: string]: string | undefined
}
