import http from '@utils/http'
import { URL } from '@constant/totp'
import { CODE } from '@constant/error'
import api from '@utils/api'

const all = () => {
  return http.post<ITotpItemResponse[]>(URL.ALL).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_ALL))
}

const updateOrCreate = async (data: ITotpUpdateOrCreate) => {
  return await http.post(URL.UPDATE_OR_CREATE, data).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_ALL))
}

const deleteTotp = async (id: number) => {
  return await http.post(URL.DELETE, {id}).catch((e) => api.resolveReject(e, CODE.HTTP_API_TOTP_ALL))
}

export default { all, updateOrCreate, deleteTotp }
