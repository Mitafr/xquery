const BACKEND_URL: string = import.meta.env.VITE_BACKEND_URL;

export const LoginPaths = {
  POST_LOGIN: BACKEND_URL + "/login",
  POST_LOGOUT: BACKEND_URL + "/logout",
}

export const AdminPaths = {
  GET_LOGS: BACKEND_URL + "/api/logs",
}
