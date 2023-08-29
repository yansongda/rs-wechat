import { CODE, MESSAGE } from "@constant/error"
import { HttpApiError } from "@models/error"

const resolveReject = (e: any, code: number, message?: string) => {
  // 只处理 HttpApiError 的转换为内部 code，其它错误原样返回
  if (e instanceof HttpApiError) {
    // 如果错误信息是默认的 http 业务错误信息，转换为 code 的（防止有些后端 api 未返回正常的 code, message 字段）
    if (e.message == MESSAGE[CODE.HTTP_API]) {
      e.message = MESSAGE[code as number]
    }

    e = new HttpApiError(code, message || e.message)
  }

  return Promise.reject(e)
}

export default { resolveReject }