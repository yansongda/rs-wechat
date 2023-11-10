import api from '@api/shorturl'
import type { FormSubmit } from 'miniprogram/types/wechat'

interface FormData {
  link: string
}

Page({
  data: {
    toptipError: '',
    link: '',
    short: ''
  },
  async submit(e: FormSubmit<FormData>) {
    await wx.showLoading({ title: '生成中', mask: true })

    try {
      const { short } = await api.create(e.detail.value.link)

      this.setData({ short })
    } catch (e: unknown) {
      this.setData({ toptipError: e instanceof Error ? e.message : '未知异常' })
    }

    await wx.hideLoading()
  },
  async copy() {
    if (this.data.short == '') {
      return
    }

    await wx.setClipboardData({ data: this.data.short })
  }
})
