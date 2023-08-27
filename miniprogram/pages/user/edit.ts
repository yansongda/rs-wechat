import {default as api} from '@api/user'
import {default as utils} from '@utils/user'

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
  onChooseAvatar(e: any) {
    this.setData({avatar: e.detail.avatarUrl})
  },
  submit(e: any) {
    const updated = e.detail.value

    api.update(updated).then(() => {
      utils.updateUser()

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
    })
  },
})

export default {}