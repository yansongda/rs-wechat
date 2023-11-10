import { STORAGE } from '@constant/app'
import userApi from '@api/user'
import userUtils from '@utils/user'
import { EE, WeixinError } from '@models/error'
import { CODE, MESSAGE } from '@constant/error'
import { DEFAULT } from '@constant/user'
import logger from '@utils/logger'
import type { GlobalData } from './types/app'
import type { LoginResponse, UpdateResult, User } from './types/user'
import type {
  AppOnUnhandledRejection,
  WxGetUpdateManagerOnCheckForUpdateResult
} from './types/wechat'

App<GlobalData>({
  globalData: {
    user: {
      avatar: DEFAULT.avatar,
      nickname: DEFAULT.nickname,
      slogan: DEFAULT.slogan,
      openId: ''
    }
  },
  async onLaunch() {
    try {
      const storage = await wx.getStorage({ key: STORAGE.USER })

      this.globalData.user = storage.data

      return
    } catch (e) {
      /* empty */
    }

    wx.login({
      success: async (res) => {
        const loginResponse: LoginResponse = await userApi.login(res.code)

        // 初始化时 app 并没有加载完成，而下面的 userUtils.sync 中会调用 user.detail 去获取用户详细信息
        // 而调用获取用户详细信息需要有用户 openId，但此时 App 又没有加载完，所以
        // 此时不能从全局数据里拿数据，所以这个时候就只能从 stroage 里拿数据，这里 storage 就是这个用的
        await wx
          .setStorage({ key: STORAGE.OPEN_ID, data: loginResponse.open_id })
          .catch(() => Promise.reject(new WeixinError(CODE.WEIXIN_STORAGE_SET)))

        const updateResult: UpdateResult = await userUtils.sync()
        if (!updateResult.isGlobalDataUpdated) {
          this.globalData.user = updateResult.user as User
        }
      },
      fail: async () => Promise.reject(new WeixinError(CODE.WEIXIN_LOGIN))
    })
  },
  onShow() {
    const updateManager = wx.getUpdateManager()

    updateManager.onCheckForUpdate((res: WxGetUpdateManagerOnCheckForUpdateResult) => {
      if (res.hasUpdate) {
        logger.info('小程序有最新版本，后续将自动更新')
      }
    })

    updateManager.onUpdateReady(() => {
      wx.showModal({
        title: '更新提示',
        content: '新版本已经准备好，是否重启应用？',
        success(res) {
          if (res.confirm) {
            updateManager.applyUpdate()
          }
        }
      })
    })

    updateManager.onUpdateFailed(() => {
      logger.error('小程序更新下载异常')
    })
  },
  onError(e: string) {
    logger.error('小程序异常', e)

    wx.showToast({ title: '小程序异常', icon: 'error' })
  },
  onUnhandledRejection(e: AppOnUnhandledRejection) {
    if (e.reason instanceof EE) {
      wx.showToast({ title: e.reason.message || MESSAGE[e.reason.code], icon: 'error' })

      return
    }

    logger.error('未知错误', e)

    wx.showToast({ title: '未知错误', icon: 'error' })
  }
})
