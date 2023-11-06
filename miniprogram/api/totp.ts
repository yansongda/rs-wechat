import http from '@utils/http'
import { URL } from '@constant/totp'
import { CODE } from '@constant/error'
import { HttpError } from '@models/error'
import logger from '@utils/logger'

const all = async () => {
  try {
    return await http.post<ITotpItemResponse[]>(URL.ALL)
  } catch (e: any) {
    logger.error('查询 TOTP 列表失败', e)
    
    throw new HttpError(CODE.HTTP_API_TOTP_ALL, e.message)
  }
}

const detail = async (id: number) => {
  try {
    return await http.post<ITotpItemResponse>(URL.DETAIL, {id} as ITotpDetailRequest)
  } catch (e: any) {
    logger.error('查询 TOTP 详情失败', e)
    
    throw new HttpError(CODE.HTTP_API_TOTP_DETAIL, e.message)
  }
}

const create = async (uri: string) => {
  try {
    return await http.post<ITotpResponse>(URL.CREATE, {uri} as ITotpCreateRequest)
  } catch (e: any) {
    logger.error('创建 TOTP 失败', e)
    
    throw new HttpError(CODE.HTTP_API_TOTP_CREATE, e.message)
  }
}

const update = async (data: ITotpUpdateRequest) => {
  try {
    return await http.post<ITotpResponse>(URL.UPDATE, data)
  } catch (e: any) {
    logger.error('更新 TOTP 信息失败', e)
    
    throw new HttpError(CODE.HTTP_API_TOTP_UPDATE, e.message)
  }
}

const deleteTotp = async (id: number) => {
  try {
    return await http.post<ITotpResponse>(URL.DELETE, {id} as ITotpDeleteRequest)
  } catch (e: any) {
    logger.error('删除 TOTP 失败', e)
    
    throw new HttpError(CODE.HTTP_API_TOTP_ALL, e.message)
  }
}

export default { all, detail, create, update, deleteTotp }
