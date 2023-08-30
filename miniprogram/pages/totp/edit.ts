Page({
  data: {
    issuer: 'Google',
    username: 'me@yansongda.cn'
  },
  onShow() {

  },
  submit() {
    wx.navigateBack()
  },
  cancel() {
    wx.navigateBack()
  }
})