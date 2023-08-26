interface IUser {
  openId: string,
  avatar: string,
  nickname: string,
  slogan: string
}

interface IUserUpdate extends IJson {
  avatar: string,
  nickname: string,
  slogan: string,
}

interface IUserUpdateResult {
  isGlobalDataUpdated: boolean,
  user?: IUser,
}

interface IUserLoginResponse {
  open_id: string,
}

interface IUserDetailResponse {
  open_id: string,
  avatar: string,
  nickname: string,
  slogan: string,
}
