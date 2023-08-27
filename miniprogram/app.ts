import { STORAGE } from '@constant/app'
import userApi from '@api/user'
import userUtils from '@utils/user'
import { HttpApiError } from '@models/error'

App<IGlobalData>({
  globalData: {
    user: {
      avatar: '/images/default-avatar.png',
      nickname: '微信用户',
      slogan: '让科技更便利',
      openId: '',
    }
  },
  onLaunch() {
    const storage = wx.getStorageSync(STORAGE.USER)
    if (storage) {
      this.globalData.user = storage

      return;
    }

    wx.login({
      success: (res) => {
        userApi.login(res.code)
          .then((res: IUserLoginResponse) => {
            // 初始化时 app 并没有加载完成，调用 updateUser 需要读取用户 openId
            // 此时不能从全局数据里拿数据，所以初始化的时候从 stroage 里拿数据
            wx.setStorageSync(STORAGE.OPEN_ID, res.open_id)

            return userUtils.sync()
          })
          .then((result: IUserUpdateResult) => {
            if (!result.isGlobalDataUpdated) {
              this.globalData.user = result.user as IUser
            }
          })
          .catch(() => wx.showToast({title: '登录失败',icon: 'error', duration: 1500, mask: true}))
      },
      fail: () => wx.showToast({title: '登录失败',icon: 'error', duration: 1500, mask: true}),
    })
  },
  onError() {
    wx.showToast({title: '小程序异常', icon: 'error', duration: 2000})
  },
  onUnhandledRejection(e: any) {
    if (e.reason instanceof HttpApiError) {
      wx.showToast({title: e.reason.message, icon: 'error', duration: 2000})
    } else {
      wx.showToast({title: '未知错误', icon: 'error', duration: 2000})
    }
  }
})