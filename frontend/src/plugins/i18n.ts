import { createI18n, type I18nOptions } from 'vue-i18n';
import en from '../locales/en.json';
import fi from '../locales/fi.json';

const messages: I18nOptions['messages'] = {
    en,
    fi,
};

export default createI18n({
    legacy: false,
    locale: 'fi', // Set the default locale
    fallbackLocale: 'en', // Set the fallback locale
    messages,
});
