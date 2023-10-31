import http from '@utils/http'
import { URL } from '@constant/totp'
import { CODE } from '@constant/error'
import api from '@utils/api'

const all = async () => {
  try {
    return await http.post<ITotpItemResponse[]>(URL.ALL)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_TOTP_ALL)
  }
}

const detail = async (id: number) => {
  try {
    return await http.post<ITotpItemResponse>(URL.DETAIL, {id} as ITotpDetailRequest)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_TOTP_DETAIL)
  }
}

const create = async (uri: string) => {
  try {
    return await http.post<ITotpResponse>(URL.CREATE, {uri} as ITotpCreateRequest)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_TOTP_CREATE)
  }
}

const update = async (data: ITotpUpdateRequest) => {
  try {
    return await http.post<ITotpResponse>(URL.UPDATE, data)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_TOTP_UPDATE)
  }
}

const deleteTotp = async (id: number) => {
  try {
    return await http.post<ITotpResponse>(URL.DELETE, {id} as ITotpDeleteRequest)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_TOTP_ALL)
  }
}

export default { all, detail, create, update, deleteTotp }
