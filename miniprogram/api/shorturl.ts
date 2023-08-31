import http from '@utils/http'
import { URL } from '@constant/shortlink'
import { CODE } from '@constant/error'
import api from '@utils/api'

const create = (link: string) => {
  return http.post<IShortlinkCreateResponse>(URL.CREATE, { link } as IShortlinkCreateRequest).catch((e) => api.resolveReject(e, CODE.HTTP_API_SHORTLINK_CREATE))
}

export default { create }