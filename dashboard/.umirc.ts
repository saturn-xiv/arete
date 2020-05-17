import { defineConfig } from 'umi';

export default defineConfig({
  nodeModulesTransform: {
    type: 'none',
  },
  base: '/my/',
  locale: { default: 'en-US' },
  hash: true,
  dynamicImport: { loading: '@/Loading' },
  antd: false,
  layout: false,
  favicon: '/favicon.ico',
});
