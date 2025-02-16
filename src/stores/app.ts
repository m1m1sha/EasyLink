export const useAppStore = defineStore('appStore', () => {
  return {}
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot))
