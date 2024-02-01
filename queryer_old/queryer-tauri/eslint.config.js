import antfu from '@antfu/eslint-config'

export default antfu(
  {
    react: true,
    formatters: true,
  },
  {
    rules: {
      'react-refresh/only-export-components': 0,
      'react/prop-types': 0,
      'node/prefer-global/process': 0,
      'react-hooks/exhaustive-deps': 2,
    },
  },
)
