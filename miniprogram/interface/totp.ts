interface ITotpItem extends ITotpItemResponse {
  isTouchMove: boolean,
}

interface ITotpItemResponse {
  id: number,
  issuer: string,
  username: string,
  code: string,
}

interface ITotpDetailRequest extends IRequestData {
  id: number,
}

interface ITotpUpdateRequest extends IRequestData{
  issuer: string,
  username: string,
}

interface ITotpCreateRequest extends IRequestData{
  uri: string
}

interface ITotpDeleteRequest extends IRequestData {
  id: number,
}

interface ITotpResponse {
}
