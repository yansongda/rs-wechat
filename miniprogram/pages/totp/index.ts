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
    remainSeconds: 30,
    items: [] as Item[],
    intervalIdentity: 0,
    isScanQrCode: false
  },
  async onShow() {
    this.timing()

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
  timing() {
    let remainSeconds = 30 - new Date().getSeconds()
    if (remainSeconds < 0) {
      remainSeconds += 30
    }

    this.setData({ remainSeconds })

    this.data.intervalIdentity =
      this.data.intervalIdentity ||
      setInterval(async () => {
        let remainSeconds = this.data.remainSeconds

        remainSeconds -= 1
        if (remainSeconds <= 0) {
          remainSeconds = 30
        }

        this.setData({ remainSeconds })

        if (remainSeconds == 30) {
          await this.all()
        }
      }, 1000)
  },
  async all() {
    await wx.showLoading({ title: '加载中' })

    api
      .all()
      .then((response) => {
        this.setData({ items: response })
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

    const scan = await wx
      .scanCode({ scanType: ['qrCode'] })
      .catch(() => Promise.reject(new WeixinError(CODE.WEIXIN_QR_CODE)))

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
  clearInterval() {
    clearInterval(this.data.intervalIdentity)

    this.data.intervalIdentity = 0
  }
})

export default {}
