import type { UseModule } from '~/types/modules'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

export const install: UseModule = (app) => {
  const pinia = createPinia()
  pinia.use(piniaPluginPersistedstate)
  app.use(pinia)
}
