const app = getApp<IGlobalData>();

const openId = app?.globalData.user.openId ?? ''

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

  if (openId != '') {
    headers.open_id = openId
  }

  return headers
}

const request = <T>(url: string, options: IOptions, mustOpenId?: boolean) => {
  return new Promise<T>((resolve, reject) => {
    const uri = formatUrl(url, options.query)
    const headers = formatHeaders(options.headers)

    if ((mustOpenId ?? true) && openId == '') {
      wx.showToast({
        title: "请重新登录",
        icon: "error",
        duration: 3000,
      })

      reject('请重新登录')
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
      },
      fail: (err) => {
        console.log(err)

        wx.showToast({
          title: "网络请求失败",
          icon: "error",
          duration: 3000,
        })

        reject(err)
      },
    })
  })
}

const post = <T>(url: string, json?: IJson, mustOpenId?: boolean) => {
  return request<T>(url, { json }, mustOpenId)
}

const get = <T>(url: string, query?: IQuery, mustOpenId?: boolean) => {
  return request<T>(url, { query }, mustOpenId)
}

export default { request, post, get }
