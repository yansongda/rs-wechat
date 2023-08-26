import {default as api} from '@utils/http'

const login = async (code: string) => {
  return await api.post<IUserLoginResponse>('/api/v1/app/login', {code}, false)
}

const detail = async (openId: string) => {
  return await api.get<IUserDetailResponse>('/api/v1/user/detail', {open_id: openId})
}

export default { login, detail }