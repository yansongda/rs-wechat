import api from '@api/totp'
import { CODE } from '@constant/error'
import { WeixinError } from '@models/error'
import type { Item } from 'miniprogram/types/totp'
import type { WeuiSlideviewButtonTap } from 'miniprogram/types/wechat'

interface Dataset {
  id: string
}

Page({
  data: {
    toptipError: '',
    slideViewButtons: [{ text: '备注' }, { type: 'warn', text: '删除' }],
    isScanQrCode: false,
    items: [] as Item[],
  },
  async onShow() {
    if (this.data.isScanQrCode) {
      this.data.isScanQrCode = false

      return
    }

    await this.all()
  },
  onHide() {
    this.clearInterval()
  },
  onUnload() {
    this.clearInterval()
  },
  async all() {
    await wx.showLoading({ title: '加载中' })

    api
      .all()
      .then((response) => {
        this.setData({ items: response })
        this.timing()
      })
      .catch((e) => {
        this.setData({ toptipError: e.message })
      })
      .finally(() => {
        wx.hideLoading().catch()
      })
  },
  async create() {
    this.data.isScanQrCode = true

    const scan = await wx.scanCode({ scanType: ['qrCode'] }).catch(() => {
      throw new WeixinError(CODE.WEIXIN_QR_CODE)
    })

    this.data.isScanQrCode = false

    api
      .create(scan.result)
      .catch(async (e) => {
        this.setData({ toptipError: e.message })
      })
      .finally(async () => {
        await this.all()
      })
  },
  async edit(id: number) {
    this.clearInterval()

    await wx.navigateTo({ url: '/pages/totp/edit?id=' + id })
  },
  async delete(id: number) {
    const result = await wx.showModal({ title: '是否确定删除？', content: '删除后数据不可恢复' })

    if (result.cancel) {
      return
    }

    api
      .deleteTotp(id)
      .catch((e: unknown) => {
        this.setData({ toptipError: e instanceof Error ? e.message : '未知异常' })
      })
      .finally(async () => {
        await this.all()
      })
  },
  async slideviewButtonTap(e: WeuiSlideviewButtonTap<Dataset, unknown>) {
    const id = Number(e.currentTarget.dataset.id)

    switch (e.detail.index) {
      case 0:
        // 备注
        await this.edit(id)
        break
      case 1:
        // 删除
        await this.delete(id)
        break
    }
  },
  timing() {
    this.data.items = this.data.items.map(item => {
      item.intervalIdentity = setInterval(async () => {
        item.period = item.period ?? 30

        item.remainSeconds = item.period - new Date().getSeconds()
        if (item.remainSeconds < 0) {
          item.remainSeconds += item.period
        }

        item.remainSeconds -= 1
        if (item.remainSeconds <= 0) {
          item.remainSeconds = item.period
        }

        if (item.remainSeconds == item.period) {
          await this.all()
        }
      }, 1000)

      return item
    });
  },
  clearInterval() {
    this.data.items = this.data.items.map(item => {
      if (item.intervalIdentity) {
        clearInterval(item.intervalIdentity)
      }

      item.intervalIdentity = 0

      return item
    })
  }
})

export default {}
