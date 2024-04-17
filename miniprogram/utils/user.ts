import api from '@api/user'
import { STORAGE } from '@constant/app'
import { DEFAULT } from '@constant/user'
import type { User } from '@types/user'

const sync = async (): Promise<User> => {
  const detail = await api.detail()

  const user: User = {
    avatar: detail.avatar || DEFAULT.avatar,
    nickname: detail.nickname || DEFAULT.nickname,
    slogan: detail.slogan || DEFAULT.slogan
  }

  wx.setStorageSync(STORAGE.USER, user)

  return user
}

export default { sync }
