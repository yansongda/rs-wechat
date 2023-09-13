import http from '@utils/http'
import { URL } from '@constant/shortlink'
import { CODE } from '@constant/error'
import api from '@utils/api'

const create = async (link: string) => {
  try {
    return await http.post<IShortlinkCreateResponse>(URL.CREATE, {link} as IShortlinkCreateRequest)
  } catch (e) {
    return await api.resolveReject(e, CODE.HTTP_API_SHORTLINK_CREATE)
  }
}

export default { create }