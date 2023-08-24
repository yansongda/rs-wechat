Page({
  data: {
    tools: [
      {
        "name": "TOTP",
        "icon": "lock",
        "url": "/pages/tools/totp/index",
      },
      {
        "name": "短链服务",
        "icon": "link",
        "url": "/pages/tools/shorturl/index",
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