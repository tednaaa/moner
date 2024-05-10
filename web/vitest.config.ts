import { fileURLToPath } from 'node:url'
import { mergeConfig, defineConfig, configDefaults } from 'vitest/config'
import viteConfig from './vite.config'

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      root: fileURLToPath(new URL('./', import.meta.url)),
      exclude: [...configDefaults.exclude, 'e2e/*'],
      environment: 'jsdom',
      globals: true,
      clearMocks: true,
      restoreMocks: true,
      setupFiles: './vitest.setup.ts'
    }
  })
)
