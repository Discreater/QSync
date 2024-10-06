// eslint.config.js
import antfu from '@antfu/eslint-config';

export default await antfu({ vue: true, typescript: true, ignores: [
  '**/generated/**',
  '**/target/**',
] }, {
  rules: {
    'style/brace-style': [
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
    'style/quotes': [
      'error',
      'single',
      {
        avoidEscape: true,
      },
    ],
    'unused-imports/no-unused-vars': [
      'error',
      {
        caughtErrorsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_',
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
      },
    ],
    'ts/no-unused-vars': [
      'error',
      {
        caughtErrorsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_',
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
      },
    ],
  },
});
