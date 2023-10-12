import type { KyInstance } from '../../node_modules/ky/distribution/types/ky.js';
import ky from 'ky';

const tokenKey = 'jwt';

/** 把localStorage裡的jwt換成傳入的token */
export const setJwt = (token: string) => {
  localStorage.setItem(tokenKey, token);
}

/** 清除既存的裡的jwt */
export const cleanJwt = () => {
  localStorage.removeItem(tokenKey);
}

/** 判斷Jwt是否過期 */
const isJwtExpired = (jwt : string) => {
  const jwtExpiry = JSON.parse(atob(jwt.split('.')[1])).exp * 1000;
  const now = new Date().getTime();
  return now > jwtExpiry;
}

/** 從localStorage取得jwt，如果過期就清掉  */
export const getJwt = () => {
  let jwt = localStorage.getItem(tokenKey);
  if (jwt && !isJwtExpired(jwt)) {
    return jwt;
  }
  localStorage.removeItem(tokenKey);
  return null;
}

export const httpClient = (): KyInstance =>
  ky.create({
    prefixUrl: import.meta.env.VITE_API_BASE_URL, // 取用env變數
    throwHttpErrors: false, // 4XX及5XX ky會拋Error，但我們要拿body的訊息
    hooks: {
      beforeRequest: [
        request => {
          const jwt = getJwt();
          if (jwt) {
            let token = `Bearer ${jwt}`;
            request.headers.set('Authorization', token);
          }
        }
      ]
    }
  });
