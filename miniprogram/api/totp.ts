import http from '@utils/http'
import { URL } from '@constant/totp'
import { CODE } from '@constant/error'
import api from '@utils/api'

const all = () => {
  return http.post<ITotpItemResponse[]>(URL.ALL).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_ALL))
}

const detail = (id: number) => {
  return http.get<ITotpItemResponse>(URL.DETAIL, {id} as ITotpDetailRequest).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_DETAIL))
}

const create = (uri: string) => {
  return http.post<ITotpResponse>(URL.UPDATE_OR_CREATE, { uri } as ITotpUpdateCreateRequest).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_CREATE))
}

const update = (data: ITotpUpdateRequest) => {
  return http.post<ITotpResponse>(URL.UPDATE_OR_CREATE, data ).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_UPDATE))
}

const deleteTotp = (id: number) => {
  return http.post<ITotpResponse>(URL.DELETE, {id}).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_ALL))
}

export default { all, detail, create, update, deleteTotp }
