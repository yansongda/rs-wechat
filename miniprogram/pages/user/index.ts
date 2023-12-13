import utils from '@utils/user'
import type { GlobalData } from 'miniprogram/types/app'

const app = getApp<GlobalData>()

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
