import {default as user} from '@api/user'
import {default as appConstant} from '@constant/app'

const app = getApp<IGlobalData>()

const updateUser = (): IUserUpdateResult => {
  let result: IUserUpdateResult = {isGlobalDataUpdated: true}

  user.detail().then((detail: IUserDetailResponse) => {
    const user: IUser = {
      avatar: detail.avatar,
      nickname: detail.nickname,
      slogan: detail.slogan,
      openId: detail.open_id,
    }

    wx.setStorageSync(appConstant.STORAGE.USER, user)

    if (app) {
      app.globalData.user = user

      return;
    }

    result.isGlobalDataUpdated = false
    result.user = user
  })

  return result
}

export default { updateUser }