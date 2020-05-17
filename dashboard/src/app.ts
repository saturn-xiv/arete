import Cookies from 'js-cookie';
import { initializeIcons } from 'office-ui-fabric-react/lib/Icons';

initializeIcons();

const fetchRoles = async function() {
  console.log('TODO fetch roles');
  return { roles: ['member'] };
};

export async function getInitialState() {
  const data = await fetchRoles();
  return data;
}

const LOCALE = 'locale';

export const locale = {
  getLocale() {
    return localStorage.getItem(LOCALE) || Cookies.get(LOCALE) || 'en-US';
  },
  setLocale({ lang, updater }: any) {
    localStorage.setItem(LOCALE, lang);
    Cookies.set(LOCALE, lang, {
      expires: 1 << 16,
      path: '/',
    });
    updater();
  },
};
