import http from '@utils/http'
import {default as constant} from '@constant/totp'

const all = async () => {
  return await http.post<ITotpItemResponse[]>(constant.URL.ALL);
}

const updateOrCreate = async (data: ITotpUpdateOrCreate) => {
  return await http.post(constant.URL.UPDATE_OR_CREATE, data)
}

const deleteTotp = async (id: number) => {
  return await http.post(constant.URL.DELETE, {id})
}

export default { all, updateOrCreate, deleteTotp }
