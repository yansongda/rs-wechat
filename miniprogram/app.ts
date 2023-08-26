import {default as constant} from '@constant/app'
import {default as user} from '@api/user'
import {default as utils} from '@utils/user'

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
    const storage = wx.getStorageSync(constant.STORAGE_USER)
    if (storage) {
      this.globalData.user = storage

      return;
    }

    wx.login({
      success: (res) => {
        user.login(res.code)
          .then((res: IUserLoginResponse) => {
            return utils.updateUser(res.open_id)
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
  }
})