import { URL } from '@constant/app'
import { CODE, WECHAT_MESSAGE } from '@constant/error'
import { HttpError, LoginError } from '@models/error'
import userUtils from '@utils/user'
import logger from '@utils/logger'

const formatUrl = (request: IRequest): void => {
  if (typeof request.query != 'undefined') {
    const query = request.query

    const paramsArray = <any>[]

    Object.keys(request.query).forEach(
      (key) => query[key] && paramsArray.push(`${key}=${query[key]}`)
    )

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

  request.headers.authorization = 'Bearer ' + openId
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
  logger.info('请求接口', request.url.indexOf('users/update') === -1 ? request : '用户更新')

  return new Promise<T>((resolve, reject) => {
    wx.request({
      url: request.url,
      data: request.data || {},
      header: request.headers ?? {},
      timeout: request.timeout || 3000,
      method: request.method || 'GET',
      success: (res: any) => {
        logger.info('接口请求成功', request.url.indexOf('users/detail') === -1 ? res : '用户详情')

        if (Number(res.data.code) === 0) {
          resolve(res.data.data)
        }

        reject(new HttpError(parseInt(res.data.code), res.data.message as string))
      },
      fail: (err: any) => {
        logger.warning('接口请求失败', err)

        reject(
          new HttpError(
            err.errno,
            WECHAT_MESSAGE[err.errno as keyof typeof WECHAT_MESSAGE] ||
              '接口请求失败：' + err.errMsg
          )
        )
      }
    })
  })
}

const wxUpload = <T>(request: IRequest) => {
  logger.info('请求上传接口', request.url, request.headers)

  return new Promise<T>((resolve, reject) => {
    const filePath: string = request.data?.filePath ?? ''
    const name: string = request.data?.name ?? ''
    const formData: IRequestData = request.data ?? {}

    if (!filePath || !name) {
      reject(new HttpError(CODE.HTTP_PARAMS))
    }

    delete formData.filePath
    delete formData.name

    wx.uploadFile({
      url: request.url,
      filePath,
      name,
      formData,
      header: request.headers ?? {},
      timeout: request.timeout || 10000,
      success: (res: any) => {
        logger.info('接口请求成功', res)

        if (res.data.code == 0) {
          resolve(res.data.data)
        }

        reject(new HttpError(parseInt(res.data.code), res.data.message as string))
      },
      fail: (err) => {
        logger.warning('接口请求失败', err)

        reject(new HttpError(undefined, '接口请求失败：' + err.errMsg))
      }
    })
  })
}

const post = <T>(
  url: string,
  data?: IRequestData,
  isUploadFile?: boolean,
  mustOpenId?: boolean
): Promise<T> => {
  return request<T>({ url, data, isUploadFile, method: 'POST' } as IRequest, mustOpenId)
}

const get = <T>(url: string, query?: IRequestQuery, mustOpenId?: boolean): Promise<T> => {
  return request<T>({ url, query, method: 'GET' } as IRequest, mustOpenId)
}

export default { request, post, get }
