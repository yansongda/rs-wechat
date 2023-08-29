interface ITotpItem extends ITotpItemResponse {
  isTouchMove: boolean,
}

interface ITotpUpdateOrCreate extends IRequestData{
  issuer?: string,
  username?: string,
  uri?: string,
}

interface ITotpItemResponse {
  id: number,
  issuer: string,
  username: string,
  code: string,
}
