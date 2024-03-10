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
  submit(e: FormSubmit<FormData>) {
    Toast({
      message: '生成中...', theme: 'loading', duration: 5000, 
      direction: 'column', preventScrollThrough: true
    })

    api
      .create(e.detail.value.link)
      .then((response: CreateResponse) => {
        this.setData({ short: response.short })

        Toast({
          message: '生成成功', theme: 'success', duration: 100,
          direction: 'column',
        })
      })
      .catch((e: HttpError) => {
        Toast({
          message: '生成失败', theme: 'error', duration: 100,
          direction: 'column'
        })

        Message.error({
          content: e.message, duration: 5000,
          offset: [20, 32], context: this,
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
