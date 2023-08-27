import { STORAGE } from '@constant/app'
import userApi from '@api/user'
import userUtils from '@utils/user'
import { EError, WeixinError } from '@models/error'
import { CODE } from '@constant/error'

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
      success: async (res) => {
        const loginResponse: IUserLoginResponse = await userApi.login(res.code)
        
        // 初始化时 app 并没有加载完成，调用 updateUser 需要读取用户 openId
        // 此时不能从全局数据里拿数据，所以初始化的时候从 stroage 里拿数据
        wx.setStorageSync(STORAGE.OPEN_ID, loginResponse.open_id)

        const updateResult: IUserUpdateResult = await userUtils.sync()
        if (!updateResult.isGlobalDataUpdated) {
          this.globalData.user = updateResult.user as IUser
        }
      },
      fail: async () => { return Promise.reject(new WeixinError(CODE.WEIXIN_LOGIN)) },
    })
  },
  onError(e) {
    wx.showToast({title: '小程序异常', icon: 'error', duration: 2000})
  },
  onUnhandledRejection(e: any) {
    if (e.reason instanceof EError) {
      wx.showToast({title: e.reason.message, icon: 'error', duration: 2000})
    } else {
      wx.showToast({title: '未知错误', icon: 'error', duration: 2000})
    }
  }
})