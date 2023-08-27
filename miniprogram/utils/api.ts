import { HttpApiError } from "@models/error"

const resolveReject = (e: any, code?: number, message?: string) => {
  if (e instanceof HttpApiError) {
    e = new HttpApiError(code, e.message || message)
  }

  return Promise.reject(e)
}

export default { resolveReject }