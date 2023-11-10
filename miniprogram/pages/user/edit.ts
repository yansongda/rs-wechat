import api from '@api/user'
import utils from '@utils/user'
import type { GlobalData } from 'miniprogram/types/app'
import type { UpdateRequest } from 'miniprogram/types/user'
import type {
  ChooseAvatarButtonTap,
  FormSubmit,
  WxGetFileSystemManagerReadFileSuccess
} from 'miniprogram/types/wechat'

interface FormData {
  avatar: string
  nickname: string
  slogan: string
}

const app = getApp<GlobalData>()

Page({
  data: {
    toptipError: '',
    avatar: app.globalData.user.avatar,
    nickname: app.globalData.user.nickname,
    slogan: app.globalData.user.slogan
  },
  onShow() {
    this.setData({
      avatar: app.globalData.user.avatar,
      nickname: app.globalData.user.nickname,
      slogan: app.globalData.user.slogan
    })
  },
  async onChooseAvatar(e: ChooseAvatarButtonTap<unknown, unknown>) {
    await wx.showLoading({ title: '上传中', icon: 'loading', mask: true })

    const res = await wx.compressImage({ src: e.detail.avatarUrl, quality: 50 })

    wx.getFileSystemManager().readFile({
      filePath: res.tempFilePath,
      encoding: 'base64',
      success: async (res: WxGetFileSystemManagerReadFileSuccess) => {
        this.setData({ avatar: 'data:image/jpeg;base64,' + res.data })

        await wx.hideLoading()
      }
    })
  },
  async submit(e: FormSubmit<FormData>) {
    await wx.showToast({ title: '更新中', icon: 'loading', mask: true })

    try {
      await api.update(e.detail.value as UpdateRequest)

      // 同步完成之后更新下全局的用户信息状态
      await utils.sync()

      await wx.showToast({ title: '修改成功', icon: 'success', mask: true })

      setTimeout(() => wx.navigateBack(), 1500)
    } catch (e: unknown) {
      await wx.hideToast()

      this.setData({ toptipError: e instanceof Error ? e.message : '未知异常' })
    }
  },
  async cancel() {
    await wx.navigateBack()
  }
})
