interface IUser {
  openId: string,
  avatar: string,
  nickname: string,
  slogan: string
}

interface IUserUpdate extends IRequestJson {
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
