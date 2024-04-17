import { DEFAULT } from '@constant/user'
import { STORAGE } from '@constant/app'
import utils from '@utils/user'
import type { User } from '@types/user'
import type { WxGetStorageSuccess } from '@types/wechat'

Page({
  data: {
    avatar: DEFAULT.avatar,
    nickname: DEFAULT.nickname,
    slogan: DEFAULT.slogan
  },
  async onLoad() {
    await utils.sync()
  },
  async onShow() {
    const storage: WxGetStorageSuccess<User> = await wx.getStorage({ key: STORAGE.USER })

    this.setData({
      avatar: storage.data.avatar,
      nickname: storage.data.nickname,
      slogan: storage.data.slogan
    })
  },
  onHide() {},
  onReady() {}
})

export default {}
