import {default as api} from '@utils/http'

const login = async (code: string) => {
  return await api.post<IUserLoginResponse>('/api/v1/app/login', {code}, false)
}

const detail = async (openId: string) => {
  return await api.get<IUserDetailResponse>('/api/v1/user/detail', {open_id: openId})
}

const update = async (updated: IUserUpdate) => {
  return await api.post('/api/v1/user/update', updated)
}

export default { login, detail, update }