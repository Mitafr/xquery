import axios, { AxiosResponse } from "axios";
import { AdminPaths } from "@/services/index";

export async function getLogs(page = 0) {
    return await axios.get(AdminPaths.GET_LOGS, {
        params: {
            page
        },
        withCredentials: true
    });
}
