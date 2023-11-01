import { STORAGE } from '@constant/app'
import userApi from '@api/user'
import userUtils from '@utils/user'
import { EError, WeixinError } from '@models/error'
import { CODE, MESSAGE } from '@constant/error'
import { DEFAULT } from '@constant/user'
import logger from '@utils/logger'

App<IGlobalData>({
  globalData: {
    user: {
      avatar: DEFAULT.avatar,
      nickname: DEFAULT.nickname,
      slogan: DEFAULT.slogan,
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
        
        // 初始化时 app 并没有加载完成，而下面的 userUtils.sync 中会调用 user.detail 去获取用户详细信息
        // 而调用获取用户详细信息需要有用户 openId，但此时 App 又没有加载完，所以
        // 此时不能从全局数据里拿数据，所以这个时候就只能从 stroage 里拿数据，这里 storage 就是这个用的
        await wx.setStorage({key: STORAGE.OPEN_ID, data: loginResponse.open_id}).catch(() => Promise.reject(new WeixinError(CODE.WEIXIN_STORAGE_SET)))

        const updateResult: IUserUpdateResult = await userUtils.sync()
        if (!updateResult.isGlobalDataUpdated) {
          this.globalData.user = updateResult.user as IUser
        }
      },
      fail: async () => Promise.reject(new WeixinError(CODE.WEIXIN_LOGIN)),
    })
  },
  onError(e: any) {
    logger.error('小程序异常', e)

    wx.showToast({title: '小程序异常', icon: 'error'})
  },
  onUnhandledRejection(e: any) {
    if (e.reason instanceof EError) {
      wx.showToast({title: e.reason.message || MESSAGE[e.reason.code], icon: 'error'})

      return;
    }

    logger.error('未知错误', e)

    wx.showToast({title: '未知错误', icon: 'error'})
  }
})