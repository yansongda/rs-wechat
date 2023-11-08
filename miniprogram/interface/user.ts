interface IUserLoginRequest extends IRequestData {
  code: string
}

interface IUserLoginResponse {
  open_id: string
}

interface IUserDetailResponse {
  open_id: string
  avatar: string
  nickname: string
  slogan: string
}

interface IUserUpdateRequest extends IRequestData {
  avatar?: string
  nickname?: string
  slogan?: string
}

interface IUserUpdateResponse {
  open_id: string
  avatar: string
  nickname: string
  slogan: string
}

interface IUserUpdateResult {
  isGlobalDataUpdated: boolean
  user?: IUser
}

interface IUser {
  openId: string
  avatar: string
  nickname: string
  slogan: string
}
