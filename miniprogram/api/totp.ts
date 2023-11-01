import http from '@utils/http'
import { URL } from '@constant/totp'
import { CODE } from '@constant/error'
import { HttpApiError } from '@models/error'
import { error } from '@utils/logger'

const all = async () => {
  try {
    return await http.post<ITotpItemResponse[]>(URL.ALL)
  } catch (e) {
    error('查询 TOTP 列表失败', e.code, e.message)
    
    return Promise.reject(new HttpApiError(CODE.HTTP_API_TOTP_ALL))
  }
}

const detail = async (id: number) => {
  try {
    return await http.post<ITotpItemResponse>(URL.DETAIL, {id} as ITotpDetailRequest)
  } catch (e) {
    error('查询 TOTP 详情失败', e.code, e.message)
    
    return Promise.reject(new HttpApiError(CODE.HTTP_API_TOTP_DETAIL))
  }
}

const create = async (uri: string) => {
  try {
    return await http.post<ITotpResponse>(URL.CREATE, {uri} as ITotpCreateRequest)
  } catch (e) {
    error('创建 TOTP 失败', e.code, e.message)
    
    return Promise.reject(new HttpApiError(CODE.HTTP_API_TOTP_CREATE))
  }
}

const update = async (data: ITotpUpdateRequest) => {
  try {
    return await http.post<ITotpResponse>(URL.UPDATE, data)
  } catch (e) {
    error('更新 TOTP 信息失败', e.code, e.message)
    
    return Promise.reject(new HttpApiError(CODE.HTTP_API_TOTP_UPDATE))
  }
}

const deleteTotp = async (id: number) => {
  try {
    return await http.post<ITotpResponse>(URL.DELETE, {id} as ITotpDeleteRequest)
  } catch (e) {
    error('删除 TOTP 失败', e.code, e.message)
    
    return Promise.reject(new HttpApiError(CODE.HTTP_API_TOTP_ALL))
  }
}

export default { all, detail, create, update, deleteTotp }
