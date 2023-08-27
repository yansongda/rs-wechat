import { URL } from '@constant/app'
import { HttpError, HttpApiError, LoginError } from '@models/error'
import userUtils from '@utils/user'

const formatUrl = (url: string, query?: IQuery): string => {
  if (!query) {
    return url
  }

  let paramsArray = [] as any

  Object.keys(query)
    .forEach(key => query[key] && paramsArray.push(`${key}=${query[key]}`))

  url += (url.search(/\?/) === -1 ? '?' : '&') + `${paramsArray.join('&')}`

  return url
}

const formatHeaders = (headers?: IHeaders): IHeaders => {
  if (!headers) {
    headers = {}
  }

  const openId = userUtils.getOpenId()

  if (openId != '') {
    headers.open_id = openId
  }

  return headers
}

const request = <T>(url: string, options: IOptions, mustOpenId?: boolean): Promise<T> => {
  return new Promise<T>((resolve, reject) => {
    const uri = URL.BASE + formatUrl(url, options.query)
    const headers = formatHeaders(options.headers)
    const openId = userUtils.getOpenId()
    
    if ((mustOpenId ?? true) && openId == '') {      
      reject(new LoginError())
    }

    wx.request({
      url: uri,
      data: options.json || options.data || {},
      header: headers,
      timeout: options.timeout || 3000,
      method: options.method || "POST",
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

const post = <T>(url: string, json?: IJson, mustOpenId?: boolean): Promise<T> => {
  return request<T>(url, { json }, mustOpenId)
}

const get = <T>(url: string, query?: IQuery, mustOpenId?: boolean): Promise<T> => {
  return request<T>(url, { query }, mustOpenId)
}

export default { request, post, get }
