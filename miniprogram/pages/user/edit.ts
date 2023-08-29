import api from '@api/user'
import utils from '@utils/user'

const app = getApp<IGlobalData>()

Page({
  data: {
    avatar: app.globalData.user.avatar,
    nickname: app.globalData.user.nickname,
    slogan: app.globalData.user.slogan,
  },
  onShow() {
    this.setData({
      avatar: app.globalData.user.avatar,
      nickname: app.globalData.user.nickname,
      slogan: app.globalData.user.slogan,
    })
  },
  async onChooseAvatar(e: any) {
    wx.showLoading({title: '上传中', mask: true})

    const { url } = await api.uploadAvatar(e.detail.avatarUrl)

    this.setData({ avatar: url })

    wx.hideLoading()
  },
  async submit(e: any) {
    await api.update(e.detail.value as IUserUpdateRequest)
    await utils.sync()

    wx.showToast({
      title: '修改成功',
      icon: 'success',
      duration: 1500,
      success: () => {
        setTimeout(() => {
          wx.navigateBack()          
        }, 1500);
      }
    })
  },
})

export default {}