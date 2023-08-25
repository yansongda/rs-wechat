Page({
  data: {
    items: [
      {
        "id": 1,
        "username": "me@yansongda.cn",
        "issuer": "Vultr",
        "code": "123 456"
      },
      {
        "id": 2,
        "username": "me@yansongda.cnaaaaaaaaaaaa",
        "issuer": "Vultr.comaaaaaaaaaaa",
        "code": "789 012"
      }
    ],
    startX: 0,
    startY: 0
  },
  touchstart: function (e: any) {
    // 开始触摸时 重置所有删除
    this.data.items.forEach(function (v: any) {
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
  touchmove: function (e: any) {    
    const index = e.currentTarget.dataset.index;
    const startX = this.data.startX;
    const startY = this.data.startY;
    const touchMoveX = e.changedTouches[0].clientX;
    const touchMoveY = e.changedTouches[0].clientY;
    
    // 获取滑动角度
    const angle = this.angle({ X: startX, Y: startY }, { X: touchMoveX, Y: touchMoveY });

    this.data.items.forEach(function (v: any, i: any) {
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
  angle: function (start: any, end: any) {
    const _X = end.X - start.X
    const _Y = end.Y - start.Y

    return 360 * Math.atan(_Y / _X) / (2 * Math.PI);
  },
})