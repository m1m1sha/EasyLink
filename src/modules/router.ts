import type { UseModule } from '~/types/modules'
import { setupLayouts } from 'virtual:generated-layouts'
import { createRouter, createWebHistory } from 'vue-router/auto'
import { routes } from 'vue-router/auto-routes'

export const install: UseModule = (app) => {
  const router = createRouter({
    history: createWebHistory(),
    routes: setupLayouts(routes),
  })
  app.use(router)
}
