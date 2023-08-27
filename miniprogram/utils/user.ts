import api from '@api/user'
import { STORAGE } from '@constant/app'

const app = getApp<IGlobalData>()

const sync = (): IUserUpdateResult => {
  let result: IUserUpdateResult = {isGlobalDataUpdated: true}

  api.detail().then((detail: IUserDetailResponse) => {
    const user: IUser = {
      avatar: detail.avatar,
      nickname: detail.nickname,
      slogan: detail.slogan,
      openId: detail.open_id,
    }

    wx.setStorageSync(STORAGE.USER, user)

    if (app) {
      app.globalData.user = user

      return;
    }

    result.isGlobalDataUpdated = false
    result.user = user
  })

  return result
}

export default { sync }