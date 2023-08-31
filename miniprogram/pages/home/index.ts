Page({
  data: {
    tools: [
      {
        "name": "TOTP身份验证器",
        "icon": "lock",
        "url": "/pages/totp/index",
      },
      {
        "name": "短链生成服务",
        "icon": "link",
        "url": "/pages/shorturl/index",
      },
      {
        "name": "时间转换",
        "icon": "time",
        "url": "/pages/time/index",
      }
    ]
  },
  navigate(e: any) {
    const { url } = e.currentTarget.dataset;

    wx.navigateTo({
      url
    })
  }
})