import http from '@utils/http'
import { URL } from '@constant/user'
import api from '@utils/api'
import { CODE } from '@constant/error'

const login = async (code: string) => {
  try {
    return await http.post<IUserLoginResponse>(URL.LOGIN, {code} as IUserLoginRequest, false, false)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_USER_LOGIN)
  }
}

const detail = async () => {
  try {
    return await http.post<IUserDetailResponse>(URL.DETAIL)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_USER_DETAIL)
  }
}

const update = async (updated: IUserUpdateRequest) => {
  try {
    return await http.post<IUserUpdateResponse>(URL.UPDATE, updated)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_USER_UPDATE)
  }
}

export default { login, detail, update }