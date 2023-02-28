
import User from './types/user';
import { postLogin, postLogout } from '../services/auth';

export const store = {
    user: new User("Anony."),

    postLogin: async function (username: string, password: string) {
        await postLogin(username, password);
        this.user.authenticate(username);
    },

    postLogout: async function () {
        await postLogout();
        this.user.logout();
    }
};