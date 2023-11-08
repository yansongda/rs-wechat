export interface Request {
  url: string
  query?: RequestQuery
  data?: RequestData
  headers?: RequestHeaders
  method?: 'POST' | 'OPTIONS' | 'GET' | 'HEAD' | 'PUT' | 'DELETE' | 'TRACE' | 'CONNECT' | undefined
  timeout?: number
  isUploadFile?: boolean
}

export interface RequestQuery {
  [key: string]: number | string | undefined
}

export interface RequestData {
  [key: string]: unknown
}

export interface RequestHeaders {
  [key: string]: string | undefined
}
