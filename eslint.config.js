// eslint.config.js
import antfu from '@antfu/eslint-config';

export default antfu({ vue: true, typescript: true, ignores: [
  '**/generated/**',
  '**/target/**',
] }, {
  rules: {
    'style/brace-style': 'off',
    'ts/brace-style': [
      'error',
      '1tbs',
      {
        allowSingleLine: true,
      },
    ],
    'style/semi': [
      'error',
      'always',
    ],
    'ts/semi': [
      'error',
      'always',
    ],
    'style/quotes': 'off',
    'ts/quotes': [
      'error',
      'single',
      {
        avoidEscape: true,
      },
    ],
    'ts/no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
      },
    ],
  },
});
