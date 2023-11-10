import api from '@api/user'
import { STORAGE } from '@constant/app'
import { DEFAULT } from '@constant/user'
import type { GlobalData } from 'miniprogram/types/app'
import type { UpdateResult, User } from 'miniprogram/types/user'

// 初始化时会调用用户详情接口，用户详情需要 openID
// 但是初始化时 app 并没有加载完成，此时不能从全局数据里拿数据
// 所以初始化的时候尝试从 storage 里拿数据
//
// 需要使用闭包，因为小程序初始化时都为空
const getOpenId = () =>
  getApp<GlobalData>()?.globalData.user.openId || wx.getStorageSync(STORAGE.OPEN_ID) || ''

const sync = async (): Promise<UpdateResult> => {
  const result: UpdateResult = { isGlobalDataUpdated: true }

  const app = getApp<GlobalData>()
  const detail = await api.detail()

  const user: User = {
    avatar: detail.avatar || DEFAULT.avatar,
    nickname: detail.nickname || DEFAULT.nickname,
    slogan: detail.slogan || DEFAULT.slogan,
    openId: detail.open_id
  }

  wx.setStorageSync(STORAGE.USER, user)

  if (app) {
    app.globalData.user = user

    return result
  }

  result.isGlobalDataUpdated = false
  result.user = user

  return result
}

export default { getOpenId, sync }
