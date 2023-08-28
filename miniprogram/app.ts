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
  async onLaunch() {
    try {
      const storage = await wx.getStorage({key: STORAGE.USER})

      this.globalData.user = storage.data

      return;
    } catch (e) {
    }

    wx.login({
      success: async (res) => {
        const loginResponse: IUserLoginResponse = await userApi.login(res.code)
        
        // 初始化时 app 并没有加载完成，调用 updateUser 需要读取用户 openId
        // 此时不能从全局数据里拿数据，所以初始化的时候从 stroage 里拿数据
        await wx.setStorage({key: STORAGE.OPEN_ID, data: loginResponse.open_id}).catch(() => Promise.reject(new WeixinError(CODE.WEIXIN_STORAGE_SET)))

        const updateResult: IUserUpdateResult = await userUtils.sync()
        if (!updateResult.isGlobalDataUpdated) {
          this.globalData.user = updateResult.user as IUser
        }
      },
      fail: async () => Promise.reject(new WeixinError(CODE.WEIXIN_LOGIN)),
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