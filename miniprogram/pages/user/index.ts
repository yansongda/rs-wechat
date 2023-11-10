import utils from '@utils/user'

const app = getApp<IGlobalData>()

Page({
  data: {
    avatar: app.globalData.user.avatar,
    nickname: app.globalData.user.nickname,
    slogan: app.globalData.user.slogan
  },
  async onLoad() {
    await utils.sync()
  },
  onShow() {
    this.setData({
      avatar: app.globalData.user.avatar,
      nickname: app.globalData.user.nickname,
      slogan: app.globalData.user.slogan
    })
  },
  onHide() {},
  onReady() {}
})

export default {}
