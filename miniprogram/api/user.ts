import {default as api} from '@utils/http'
import {default as constant} from '@constant/user'

const login = async (code: string) => {
  return await api.post<IUserLoginResponse>(constant.URL.LOGIN, {code}, false)
}

const detail = async () => {
  return await api.get<IUserDetailResponse>(constant.URL.DETAIL)
}

const update = async (updated: IUserUpdate) => {
  return await api.post(constant.URL.UPDATE, updated)
}

export default { login, detail, update }