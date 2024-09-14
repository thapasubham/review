export function checkCookie() {
      if (typeof document !== 'undefined') {
        const cookieHeader = document.cookie || '';
        const cookies = new Map(cookieHeader.split('; ').map(cookie => cookie.split('=')));
        const token = cookies.get('token');
        return { token };
      }
      return { token: null };
    }
    