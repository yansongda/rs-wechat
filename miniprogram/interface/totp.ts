interface ITotpItem extends ITotpItemResponse {
  isTouchMove: boolean,
}

interface ITotpUpdateRequest extends IRequestData{
  issuer: string,
  username: string,
}

interface ITotpItemResponse {
  id: number,
  issuer: string,
  username: string,
  code: string,
}
