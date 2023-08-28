import { HttpApiError } from "@models/error"

// 只处理 HttpApiError 的转换为内部 code，其它错误原样返回
const resolveReject = (e: any, code?: number, message?: string) => {
  if (e instanceof HttpApiError) {
    e = new HttpApiError(code, e.message || message)
  }

  return Promise.reject(e)
}

export default { resolveReject }