import http from '@utils/http'
import { URL } from '@constant/shortlink'
import { CODE } from '@constant/error'
import { HttpError } from '@models/error'
import logger from '@utils/logger'
import error from '@utils/error'
import type { CreateRequest, CreateResponse } from 'miniprogram/types/shortlink'

const create = async (link: string) => {
  try {
    return await http.post<CreateResponse>(URL.CREATE, {
      link
    } as CreateRequest)
  } catch (e: unknown) {
    logger.error('创建短链接失败', e)

    throw new HttpError(CODE.HTTP_API_SHORTLINK_CREATE, error.getErrorMessage(e))
  }
}

export default { create }
