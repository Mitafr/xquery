import axios from "axios";
import { LoginPaths } from "@/services/index";

export async function postLogin(username: string, password: string) {
  return await axios.post(LoginPaths.POST_LOGIN, {
    user: username,
    password: password,
  });
}

export async function postLogout() {
  return await axios.get(LoginPaths.POST_LOGOUT);
}
