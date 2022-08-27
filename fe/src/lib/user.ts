type UserJwt = {
    exp: number;
    id: number;
    email: string;
    username: string;
}

const parseJwt = (jwt: string): UserJwt | null => {
    try {
        return JSON.parse(atob(jwt.split('.')[1])) as UserJwt;
    } catch (err) {
        return null;
    }
}

export const getUserJwt = () => {
    const jwt = window.localStorage.getItem('jwt') || '';
    return {
        user: parseJwt(jwt),
        jwt,
    };
}