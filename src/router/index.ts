import { createRouter, createWebHistory } from 'vue-router'
import LoginPage from '../LoginPage/LoginPage.vue'
import HomePage from '../HomePage/HomePage.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomePage
    },
    {
      path: '/login',
      name: 'login',
      component: LoginPage,
      meta: { sidebar: false }
    }
  ]
})

export default router
