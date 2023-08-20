interface User {
  avatar: String,
  nickname: String,
  slogan?: String
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
      nickname: "微信用户",
      slogan: "让科技更便利"
    }
  }
})