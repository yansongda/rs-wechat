interface User {
  avatar: String,
  nickname: String,
  openId?: String,
}

interface GlobalData {
  globalData: {
    user: User,
  }
}

App<GlobalData>({
  globalData: {
    user: {
      avatar: "/images/default-avatar.png",
      nickname: "微信用户"
    }
  }
})