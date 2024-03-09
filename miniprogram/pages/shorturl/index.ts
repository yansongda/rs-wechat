import Message from 'tdesign-miniprogram/message/index';
import Toast from 'tdesign-miniprogram/toast/index';
import api from '@api/shorturl'
import { HttpError } from '@models/error'
import type { CreateResponse } from 'miniprogram/types/shortlink'
import type { FormSubmit } from 'miniprogram/types/wechat'

interface FormData {
  link: string
}

Page({
  data: {
    link: '',
    short: ''
  },
  async submit(e: FormSubmit<FormData>) {
    Toast({
      message: '生成中...',
      theme: 'loading',
      direction: 'column',
      preventScrollThrough: true,
      duration: 5000
    })

    api
      .create(e.detail.value.link)
      .then((response: CreateResponse) => {
        this.setData({ short: response.short })
      })
      .catch((e: HttpError) => {
        Message.error({
          context: this,
          offset: [20, 32],
          duration: 5000,
          content: e.message,
        })
      })
      .finally(async () => {
        Toast({
          message: '生成成功',
          theme: 'success',
          direction: 'column',
          duration: 100
        })
      })
  },
  async copy() {
    if (this.data.short == '') {
      return
    }

    await wx.setClipboardData({ data: this.data.short })
  }
})
