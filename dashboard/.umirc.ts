import { defineConfig } from 'umi';

export default defineConfig({
  nodeModulesTransform: {
    type: 'none',
  },
  base: '/my/',
  locale: { default: 'en-US' },
  antd: false,
  layout: false,
});
