import http from '@utils/http'
import { URL } from '@constant/totp'
import { CODE } from '@constant/error'
import { HttpError } from '@models/error'
import logger from '@utils/logger'
import type {
  CreateRequest,
  DeleteRequest,
  DetailRequest,
  Item,
  Response,
  UpdateRequest
} from 'miniprogram/types/totp'

const all = async () => {
  try {
    return await http.post<Item[]>(URL.ALL)
  } catch (e: unknown) {
    logger.error('查询 TOTP 列表失败', e)

    throw new HttpError(CODE.HTTP_API_TOTP_ALL, e instanceof Error ? e.message : '未知错误')
  }
}

const detail = async (id: number) => {
  try {
    return await http.post<Item>(URL.DETAIL, { id } as DetailRequest)
  } catch (e: unknown) {
    logger.error('查询 TOTP 详情失败', e)

    throw new HttpError(CODE.HTTP_API_TOTP_DETAIL, e instanceof Error ? e.message : '未知错误')
  }
}

const create = async (uri: string) => {
  try {
    return await http.post<Response>(URL.CREATE, { uri } as CreateRequest)
  } catch (e: unknown) {
    logger.error('创建 TOTP 失败', e)

    throw new HttpError(CODE.HTTP_API_TOTP_CREATE, e instanceof Error ? e.message : '未知错误')
  }
}

const update = async (data: UpdateRequest) => {
  try {
    return await http.post<Response>(URL.UPDATE, data)
  } catch (e: unknown) {
    logger.error('更新 TOTP 信息失败', e)

    throw new HttpError(CODE.HTTP_API_TOTP_UPDATE, e instanceof Error ? e.message : '未知错误')
  }
}

const deleteTotp = async (id: number) => {
  try {
    return await http.post<Response>(URL.DELETE, { id } as DeleteRequest)
  } catch (e: unknown) {
    logger.error('删除 TOTP 失败', e)

    throw new HttpError(CODE.HTTP_API_TOTP_ALL, e instanceof Error ? e.message : '未知错误')
  }
}

export default { all, detail, create, update, deleteTotp }
