const app = getApp<GlobalData>()

Page({
  data: {
    user: app.globalData.user,
  },
  onChooseAvatar(e: any) {
    const { avatarUrl } = e.detail 
    
    // todo: 上传文件到服务端并更新
    app.globalData.user.avatar = avatarUrl

    this.setData({
      user: app.globalData.user
    })
  },
  submit(e: any) {
    // todo: 提交表单，更新用户数据
    console.log(e.detail.value)
  },
})

export default {}