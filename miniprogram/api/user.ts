import http from '@utils/http'
import { URL } from '@constant/user'
import api from '@utils/api'
import { CODE } from '@constant/error'

const login = (code: string) => {
  return http.post<IUserLoginResponse>(URL.LOGIN, {code}, false).catch((e) => api.resolveReject(e, CODE.HTTP_API_USER_LOGIN))
}

const uploadAvatar = (path: string) => {

}

const detail = () => {
  return http.get<IUserDetailResponse>(URL.DETAIL).catch((e) => api.resolveReject(e, CODE.HTTP_API_USER_DETAIL))
}

const update = (updated: IUserUpdate) => {
  return http.post(URL.UPDATE, updated).catch((e) => api.resolveReject(e, CODE.HTTP_API_USER_UPDATE))
}

export default { login, uploadAvatar, detail, update }