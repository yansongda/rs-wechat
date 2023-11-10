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

    const { link } = e.detail.value

    api
      .create(link)
      .then(({ short }) => {
        this.setData({ short })
      })
      .catch((e) => {
        this.setData({ toptipError: e.message })
      })
      .finally(async () => {
        await wx.hideLoading()
      })
  },
  async copy() {
    if (this.data.short == '') {
      return
    }

    await wx.setClipboardData({ data: this.data.short })
  }
})
