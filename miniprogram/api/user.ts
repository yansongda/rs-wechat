import http from '@utils/http'
import { URL } from '@constant/user'
import { CODE } from '@constant/error'
import logger from '@utils/logger'
import { HttpError } from '@models/error'

const login = async (code: string) => {
  try {
    return await http.post<IUserLoginResponse>(URL.LOGIN, {code} as IUserLoginRequest, false, false)
  } catch (e: any) {
    logger.error('登录接口请求失败', e)

    throw new HttpError(CODE.HTTP_API_USER_LOGIN, e.message)
  }
}

const detail = async () => {
  try {
    return await http.post<IUserDetailResponse>(URL.DETAIL)
  } catch (e: any) {
    logger.error('查询用户详情失败', e)
    
    throw new HttpError(CODE.HTTP_API_USER_DETAIL, e.message)
  }
}

const update = async (updated: IUserUpdateRequest) => {
  try {
    return await http.post<IUserUpdateResponse>(URL.UPDATE, updated)
  } catch (e: any) {
    logger.error('更新用户信息失败', e)
    
    throw new HttpError(CODE.HTTP_API_USER_UPDATE, e.message)
  }
}

export default { login, detail, update }