import api from '@api/totp'
import type { UpdateRequest } from 'miniprogram/types/totp'
import type { FormSubmit, WeuiDialogTap } from 'miniprogram/types/wechat'

interface Query {
  id?: string
}

interface FormData {
  issuer: string
  username: string
}

Page({
  data: {
    toptipError: '',
    dialogShow: false,
    dialogButtons: [{ text: '取消' }, { text: '重试' }],
    id: 0,
    issuer: '',
    username: ''
  },
  onLoad(query: Query) {
    this.data.id = Number(query.id || 0)
  },
  async onShow() {
    await wx.showLoading({ title: '加载中' })

    api
      .detail(this.data.id)
      .then(({ id, issuer, username }) => {
        this.setData({ id, issuer: issuer ?? '', username: username ?? '' })
      })
      .catch(() => {
        this.setData({ dialogShow: true })
      })
      .finally(() => wx.hideLoading())
  },
  async submit(e: FormSubmit<FormData>) {
    await wx.showToast({ title: '更新中', icon: 'loading', mask: true })

    try {
        await api.update({ id: this.data.id, ...e.detail.value } as UpdateRequest)

        await wx.showToast({ title: '修改成功', icon: 'success', mask: true })

        setTimeout(() => wx.navigateBack(), 1500)
    } catch (e: unknown) {
        this.setData({ toptipError: e instanceof Error ? e.message : '未知异常' })
    }

    await wx.hideToast()
  },
  async cancel() {
    await wx.navigateBack()
  },
  async dialogTap(e: WeuiDialogTap) {
    this.setData({ dialogShow: false })

    const { index } = e.detail

    if (index === 1) {
      await this.onShow()
    }
  },
  dialogClose() {
    this.setData({ dialogShow: false })
  }
})
