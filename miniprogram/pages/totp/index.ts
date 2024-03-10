import Message from 'tdesign-miniprogram/message/index';
import Toast from 'tdesign-miniprogram/toast/index';
import api from '@api/totp'
import { CODE } from '@constant/error'
import { HttpError, WeixinError } from '@models/error'
import type { Item } from 'miniprogram/types/totp'
import { SwipeTap } from 'miniprogram/types/tdesign';

interface SwipeButton {
  text: string,
  className: string,
}

interface Dataset {
  id: string
}

Page({
  data: {
    swipeButtons: [
      { text: '备注', className: 'btn edit-btn'},
      { text: '删除', className: 'btn delete-btn' }
    ] as SwipeButton[],
    dialogVisible: false,
    dialogConfirmBtn: { content: '删除', variant: 'base' },
    dialogDataId: 0,
    intervalIdentity: -1,
    items: [] as Item[]
  },
  onShow() {
    this.setupRefreshInterval()

    this.all()
  },
  onHide() {
    this.clearRefreshInterval()
  },
  onUnload() {
    this.clearRefreshInterval()
  },
  all() {
    Toast({
      message: '加载中...', theme: 'loading', duration: 5000,
      direction: 'column', preventScrollThrough: true,
    })

    api
      .all()
      .then((response) => {
        this.setData({ items: response })

        Toast({
          message: '加载成功', theme: 'success', duration: 100,
          direction: 'column',
        })
      })
      .catch((e: HttpError) => Message.error({
        content: e.message, duration: 5000,
        offset: [20, 32], context: this,
      }))
  },
  async create() {
    const scan = await wx.scanCode({ scanType: ['qrCode'] }).catch(() => {
      throw new WeixinError(CODE.WEIXIN_QR_CODE)
    })

    api
      .create(scan.result)
      .catch((e: HttpError) => Message.error({
        content: e.message, duration: 5000,
        offset: [20, 32], context: this,
      }))
      .finally(() => this.all())
  },
  async edit(id: number) {
    this.clearRefreshInterval()

    await wx.navigateTo({ url: '/pages/totp/edit?id=' + id })
  },
  refreshCode(id: number, index: number) {
    api
      .detail(id)
      .then((response) => this.setData({ [`items[${index}].code`]: response.code }))
      .catch((e: HttpError) => Message.error({
        content: e.message, duration: 5000,
        offset: [20, 32], context: this,
      }))
  },
  async swipeClick(e: SwipeTap<SwipeButton, Dataset, Dataset>) {
    const id = Number(e.currentTarget.dataset.id)

    switch (e.detail.text) {
      case '备注':
        await this.edit(id)
        break
      case '删除':
        this.setData({ dialogVisible: true, dialogDataId: id })
        break
    }
  },
  dialogConfirm(e: SwipeTap<SwipeButton, Dataset, Dataset>) {
    const id = Number(e.currentTarget.dataset.id)

    api.deleteTotp(id)
      .catch((e: HttpError) => Message.error({
        content: e.message, duration: 5000,
        offset: [20, 32], context: this,
      }))
      .finally(() => this.all())
  },
  dialogCancel() {
    this.setData({ dialogVisible: false, dialogDataId: 0 })
  },
  setupRefreshInterval() {
    const intervalIdentity = setInterval(async () => {
      for (const item of this.data.items) {
        const index = this.data.items.indexOf(item)
        const period = item.period ?? 30

        let remainSeconds = period - new Date().getSeconds()
        if (remainSeconds <= 0) {
          remainSeconds += period
        }

        this.setData({ [`items[${index}].remainSeconds`]: remainSeconds })

        if (remainSeconds == period) {
          this.refreshCode(item.id, index)
        }
      }
    }, 1000)

    this.setData({ intervalIdentity })
  },
  clearRefreshInterval() {
    if (this.data.intervalIdentity > 0) {
      clearInterval(this.data.intervalIdentity)
    }

    this.setData({ intervalIdentity: -1 })
  }
})

export default {}
