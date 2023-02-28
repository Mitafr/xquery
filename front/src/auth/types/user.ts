export default class User {
    authenticated: boolean;
    username: string;
    profile: string;

    constructor(username: string) {
        this.authenticated = false;
        this.username = username;
        this.profile = "TEST";
    }

    authenticate(username: string) {
        this.authenticated = true;
        this.username = username;
    }

    logout() {
        this.authenticated = false;
    }
}