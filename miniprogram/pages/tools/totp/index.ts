import api from '@api/totp'
import { CODE } from '@constant/error'
import { WeixinError } from '@models/error'

Page({
  data: {
    remainSeconds: 30,
    items: [] as ITotpItem[],
    intervalIdentity: 0,
    startX: 0,
    startY: 0
  },
  onShow() {
    wx.showLoading({title: '加载中'})

    this.timing()
    this.all()
  },
  onHide() {
    clearInterval(this.data.intervalIdentity)
  },
  onUnload() {
    clearInterval(this.data.intervalIdentity)
  },
  timing() {
    let remainSeconds = 30 - (new Date()).getSeconds()
    if (remainSeconds < 0) {
      remainSeconds += 30
    }

    this.setData({remainSeconds})

    this.data.intervalIdentity = setInterval(() => {
      let remainSeconds = this.data.remainSeconds

      remainSeconds -= 1
      if (remainSeconds <= 0) {
        remainSeconds = 30
      }

      this.setData({ remainSeconds })

      if (remainSeconds == 30) {
        this.all()
      }
    }, 1000)
  },
  async all() {
    const response = await api.all()

    wx.hideLoading();

    const items: ITotpItem[] = []

    response.forEach((v: ITotpItemResponse) => {
      items.push({
        isTouchMove: false,
        ...v
      })
    })

    this.setData({items})
  },
  async add() {
    const scan = await wx.scanCode({scanType: ['qrCode']}).catch(() => {
      return Promise.reject(new WeixinError(CODE.WEIXIN_QR_CODE))
    })
    
    await api.updateOrCreate({uri: scan.result})
    
    await this.all()
  },
  async delete(e: any) {
    const result = await wx.showModal({title: '是否确定删除？', content: '删除后数据不可恢复'})

    if (result.confirm) {
      await api.deleteTotp(e.currentTarget.dataset.id)
      await this.all()
    }
  },
  touchstart(e: any) {
    // 开始触摸时 重置所有删除
    this.data.items.forEach(function (v: ITotpItem) {
      if (v.isTouchMove) {
        v.isTouchMove = false;
      }
    })

    // 手指触摸动作开始 记录起点X坐标
    this.setData({
      startX: e.changedTouches[0].clientX,
      startY: e.changedTouches[0].clientY,

      items: this.data.items
    })
  },
  touchmove(e: any) {    
    const index = e.currentTarget.dataset.index;
    const startX = this.data.startX;
    const startY = this.data.startY;
    const touchMoveX = e.changedTouches[0].clientX;
    const touchMoveY = e.changedTouches[0].clientY;
    
    // 获取滑动角度
    const angle = this.angle({ X: startX, Y: startY }, { X: touchMoveX, Y: touchMoveY });

    this.data.items.forEach(function (v: ITotpItem, i: number) {
      v.isTouchMove = false

      if (Math.abs(angle) > 30 || i != index) {
        return;
      }

      if (touchMoveX > startX) {
        //右滑
        v.isTouchMove = false
      } else {
        //左滑
        v.isTouchMove = true
      } 
    })

    this.setData({
      items: this.data.items
    })
  },
  angle(start: any, end: any) {
    const _X = end.X - start.X
    const _Y = end.Y - start.Y

    return 360 * Math.atan(_Y / _X) / (2 * Math.PI);
  },
})

export default {}