import http from '@utils/http'

const all = async () => {
  return await http.post<ITotpItemResponse[]>('/api/v1/totp/all');
}

const updateOrCreate = async (data: ITotpUpdateOrCreate) => {
  return await http.post('/api/v1/totp/updateOrCreate', data)
}

const deleteTotp = async (id: number) => {
  return await http.post('/api/v1/totp/delete', {id})
}

export default { all, updateOrCreate, deleteTotp }
