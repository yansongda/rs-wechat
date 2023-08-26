import {default as user} from '@api/user'
import {default as appConstant} from '@constant/app'

const app = getApp<IGlobalData>()

const updateUser = (openId: string): IUserUpdateResult => {
  let result: IUserUpdateResult = {isGlobalDataUpdated: true}

  user.detail(openId).then((detail: IUserDetailResponse) => {
    const user: IUser = {
      avatar: detail.avatar,
      nickname: detail.nickname,
      slogan: detail.slogan,
      openId: detail.open_id,
    }

    wx.setStorageSync(appConstant.STORAGE_USER, user)

    if (!app) {
      result.isGlobalDataUpdated = false
      result.user = user
    }

    app.globalData.user = user
  })

  return result
}

export default { updateUser }