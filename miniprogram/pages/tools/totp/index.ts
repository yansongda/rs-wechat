import api from '@api/totp'

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
    const scan = await wx.scanCode({scanType: ['qrCode']})
    
    await api.updateOrCreate({uri: scan.result}).catch(() => wx.showToast({title: '添加失败', icon: 'error', duration: 2000}))
    
    await this.all()
  },
  delete(e: any) {
    api.deleteTotp(e.currentTarget.dataset.id)
    .then(() => this.all())
    .catch(() => wx.showToast({title: '删除失败', icon: 'error', duration: 2000}))
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