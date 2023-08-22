const app = getApp<GlobalData>()

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
    const { avatarUrl } = e.detail 
    
    // todo: 上传文件到服务端并更新当前用户的头像
    app.globalData.user.avatar = avatarUrl

    this.setData({
      avatar: avatarUrl
    })
  },
  submit(e: any) {
    const updated = e.detail.value

    // todo: 提交表单
    console.log(updated)

    wx.showToast({
      title: '修改成功',
      icon: 'success',
      duration: 1500,
      success: () => {
        app.globalData.user.nickname = updated.nickname
        app.globalData.user.slogan = updated.slogan

        setTimeout(() => {
          wx.navigateBack()          
        }, 1500);
      }
    })
  },
})

export default {}