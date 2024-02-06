import api from '@api/shorturl'
import { HttpError } from '@models/error'
import { CreateResponse } from 'miniprogram/types/shortlink'
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

    api
      .create(e.detail.value.link)
      .then((response: CreateResponse) => {
        this.setData({ short: response.short })
      })
      .catch((e: HttpError) => {
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
