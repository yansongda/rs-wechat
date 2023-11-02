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
    await wx.showLoading({title: '上传中', icon: 'loading', mask: true})

    let res = await wx.compressImage({ src: e.detail.avatarUrl, quality: 50})

    wx.getFileSystemManager().readFile({
      filePath: res.tempFilePath,
      encoding: 'base64',
      success: async (res: any) => {      
        this.setData({ avatar: "data:image/png;base64," + res.data })

        await wx.hideLoading()
      }
    })
  },
  async submit(e: any) {
    await wx.showToast({title: '更新中', icon: 'loading', mask: true, duration: 3000})

    await api.update(e.detail.value as IUserUpdateRequest)

    // 同步完成之后更新下全局的用户信息状态
    await utils.sync()

    wx.showToast({
      title: '修改成功',
      icon: 'success',
      success: () => {
        setTimeout(() => {
          wx.navigateBack()          
        }, 1500);
      }
    })
  },
  async cancel() {
    await wx.navigateBack()
  },
})

export default {}