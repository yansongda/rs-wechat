import Message from 'tdesign-miniprogram/message/index';
import Toast from 'tdesign-miniprogram/toast/index';
import api from '@api/user'
import user from '@utils/user'
import error from '@utils/error'
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

    const res = await wx.compressImage({ src: e.detail.avatarUrl.toString(), quality: 50 })

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
    Toast({
      message: '更新中...',
      theme: 'loading',
      direction: 'column',
      preventScrollThrough: true,
      duration: 5000
    })

    try {
      await api.update(e.detail.value as UpdateRequest)

      // 同步完成之后更新下全局的用户信息状态
      await user.sync()

      Toast({
        message: '修改成功',
        theme: 'success',
        direction: 'column',
        preventScrollThrough: true,
        duration: 1500
      })

      setTimeout(() => wx.navigateBack(), 1500)
    } catch (e: unknown) {
      Message.error({
        context: this,
        offset: [20, 32],
        duration: 5000,
        content: error.getErrorMessage(e),
      })
    }
  },
  async cancel() {
    await wx.navigateBack()
  }
})
