export function load(){
    const cookieHeader = document.cookie;
    // @ts-ignore
    const cookies = new Map(cookieHeader.split('; ').map((cookie) => cookie.split('=')));
    const token = cookies.get('token');
    return token;
}