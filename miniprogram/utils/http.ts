import { URL } from '@constant/app'
import { HttpError, HttpApiError, LoginError } from '@models/error'
import userUtils from '@utils/user'

const formatUrl = (request: IRequest): void => {
  if (typeof request.query != 'undefined') {
    const query = request.query

    let paramsArray = <any>[]

    Object.keys(request.query).forEach(key => query[key] && paramsArray.push(`${key}=${query[key]}`))
  
    request.url += (request.url.search(/\?/) === -1 ? '?' : '&') + `${paramsArray.join('&')}`
  }

  if (request.url.startsWith('/')) {
    request.url = URL.BASE + request.url
  }
}

const formatHeaders = (request: IRequest, openId: string): void => {
  if (typeof request.headers == 'undefined') {
    request.headers = {}
  }

  request.headers.open_id = openId
}

const request = <T>(request: IRequest, mustOpenId?: boolean): Promise<T> => {
  const openId = userUtils.getOpenId()

  if (openId == '' && (mustOpenId ?? true)) {
    return Promise.reject(new LoginError())
  }

  formatUrl(request)
  formatHeaders(request, openId)

  if (request.isUploadFile) {
    return wxUpload(request)
  }

  return wxRequest(request)
}

const wxRequest = <T>(request: IRequest) => {
  return new Promise<T>((resolve, reject) => {
    wx.request({
      url: request.url,
      data: request.data || {},
      header: request.headers ?? {},
      timeout: request.timeout || 3000,
      method: request.method || 'POST',
      success: (res: any) => {
        if (res.data.code == 0) {
          resolve(res.data.data)
        }
  
        reject(new HttpApiError(res.data.code as number, res.data.message as string))
      },
      fail: (err) => {
        reject(new HttpError(err.errMsg))
      },
    })
  })
}

const wxUpload = <T>(request: IRequest) => {
  return new Promise<T>((resolve, reject) => {
    wx.uploadFile({
      url: request.url,
      filePath: '',
      name: '',
      header: request.headers ?? {},
      timeout: request.timeout || 10000,
      success: (res: any) => {
        if (res.data.code == 0) {
          resolve(res.data.data)
        }
  
        reject(new HttpApiError(res.data.code as number, res.data.message as string))
      },
      fail: (err) => {
        reject(new HttpError(err.errMsg))
      },
    })
  })
}

const post = <T>(url: string, data?: IRequestData, isUploadFile?: boolean, mustOpenId?: boolean): Promise<T> => {
  return request<T>({url, data, isUploadFile}, mustOpenId)
}

const get = <T>(url: string, query?: IRequestQuery, mustOpenId?: boolean): Promise<T> => {
  return request<T>({url, query}, mustOpenId)
}

export default { request, post, get }
