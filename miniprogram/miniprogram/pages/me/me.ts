const app = getApp<GlobalData>()

Page({
  data: {
    user: app.globalData.user,
    motto: 'Hello World',
  },
  onLoad() {
    // if (!app.globalData.user?.openId) {
    //   wx.login({
    //     success: res => {
    //       console.log(res.code)
    //       // 发送 res.code 到后台换取 openId, sessionKey, unionId
    //     },
    //   })
    // }
  },
  onShow() {
    this.getTabBar().init();
  },
  onChooseAvatar(e: any) {
    const { avatarUrl } = e.detail 
    
    app.globalData.user.avatar = avatarUrl;
  }
})
