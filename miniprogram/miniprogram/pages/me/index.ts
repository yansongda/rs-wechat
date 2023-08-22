const app = getApp<GlobalData>()

Page({
  data: {
    avatar: app.globalData.user.avatar,
    nickname: app.globalData.user.nickname,
    slogan: app.globalData.user.slogan,
  },
  onLoad() {
  },
  onShow() {
    this.setData({
      avatar: app.globalData.user.avatar,
      nickname: app.globalData.user.nickname,
      slogan: app.globalData.user.slogan,
    })
  },
  onHide() {
  },
  onReady() {
  }
})

export default {}
