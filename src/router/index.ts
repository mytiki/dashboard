import { createRouter, createWebHistory } from 'vue-router'
import LoginPage from '../LoginPage/LoginPage.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: LoginPage,
      meta: { sidebar: false }
    }
  ]
})

export default router
