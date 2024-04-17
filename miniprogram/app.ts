import { STORAGE } from '@constant/app'
import accessToken from '@api/accessToken'
import { EE, WeixinError } from '@models/error'
import { CODE, MESSAGE } from '@constant/error'
import logger from '@utils/logger'
import type { GlobalData } from '@types/app'
import type {
  AppOnUnhandledRejection,
  WxGetUpdateManagerOnCheckForUpdateResult,
  WxLoginSuccessCallbackResult
} from '@types/wechat'
import type { LoginResponse } from '@types/accessToken'

App<GlobalData>({
  async onLaunch() {
    try {
      await wx.checkSession()

      await wx.getStorage({ key: STORAGE.ACCESS_TOKEN })

      return
    } catch (e) {
      /* empty */
    }

    wx.login({
      success: async (res: WxLoginSuccessCallbackResult) => {
        const loginResponse: LoginResponse = await accessToken.login(res.code)

        await wx
          .setStorage({ key: STORAGE.ACCESS_TOKEN, data: loginResponse.access_token })
          .catch(() => Promise.reject(new WeixinError(CODE.WEIXIN_STORAGE_SET)))
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
  async onError(e: string) {
    logger.error('小程序异常', e)

    await wx.showToast({ title: '小程序异常', icon: 'error' })
  },
  async onUnhandledRejection(e: AppOnUnhandledRejection) {
    if (e.reason instanceof EE) {
      await wx.showToast({ title: MESSAGE[e.reason.code], icon: 'error' })

      return
    }

    logger.error('未知错误', e)

    await wx.showToast({ title: '出现未知错误', icon: 'error' })
  }
})
