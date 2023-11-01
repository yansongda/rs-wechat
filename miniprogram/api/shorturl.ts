import http from '@utils/http'
import { URL } from '@constant/shortlink'
import { CODE } from '@constant/error'
import { HttpApiError } from '@models/error'
import logger from '@utils/logger'

const create = async (link: string) => {
  try {
    return await http.post<IShortlinkCreateResponse>(URL.CREATE, {link} as IShortlinkCreateRequest)
  } catch (e) {
    logger.error('创建短链接失败', e.code, e.message)
    
    return Promise.reject(new HttpApiError(CODE.HTTP_API_SHORTLINK_CREATE))
  }
}

export default { create }