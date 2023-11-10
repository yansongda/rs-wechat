import http from '@utils/http'
import { URL } from '@constant/user'
import { CODE } from '@constant/error'
import logger from '@utils/logger'
import { HttpError } from '@models/error'
import type {
  DetailResponse,
  LoginRequest,
  LoginResponse,
  UpdateRequest,
  UpdateResponse
} from 'miniprogram/types/user'

const login = async (code: string) => {
  try {
    return await http.post<LoginResponse>(URL.LOGIN, { code } as LoginRequest, false, false)
  } catch (e: unknown) {
    logger.error('登录接口请求失败', e)

    throw new HttpError(CODE.HTTP_API_USER_LOGIN, e instanceof Error ? e.message : '未知错误')
  }
}

const detail = async () => {
  try {
    return await http.post<DetailResponse>(URL.DETAIL)
  } catch (e: unknown) {
    logger.error('查询用户详情失败', e)

    throw new HttpError(CODE.HTTP_API_USER_DETAIL, e instanceof Error ? e.message : '未知错误')
  }
}

const update = async (updated: UpdateRequest) => {
  try {
    return await http.post<UpdateResponse>(URL.UPDATE, updated)
  } catch (e: unknown) {
    logger.error('更新用户信息失败', e)

    throw new HttpError(CODE.HTTP_API_USER_UPDATE, e instanceof Error ? e.message : '未知错误')
  }
}

export default { login, detail, update }
