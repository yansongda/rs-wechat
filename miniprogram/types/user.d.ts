import { RequestData } from './http'

export interface LoginRequest extends RequestData {
  code: string
}

export interface LoginResponse {
  open_id: string
}

export interface DetailResponse {
  open_id: string
  avatar: string
  nickname: string
  slogan: string
}

export interface UpdateRequest extends RequestData {
  avatar?: string
  nickname?: string
  slogan?: string
}

export interface UpdateResponse {
  open_id: string
  avatar: string
  nickname: string
  slogan: string
}

export interface UpdateResult {
  isGlobalDataUpdated: boolean
  user?: 
}

export interface User {
  openId: string
  avatar: string
  nickname: string
  slogan: string
}
