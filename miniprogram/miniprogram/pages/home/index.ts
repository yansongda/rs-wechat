Page({
  data: {
    tools: [
      {
        "name": "TOTP",
        "icon": "lock",
        "url": "/pages/tools/totp",
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