import { createRouter, createWebHistory } from 'vue-router'
import LoginPage from '../LoginPage/LoginPage.vue'
import HomePage from '../HomePage/HomePage.vue'
import NewRequestPage from '../NewRequestPage/NewRequestPage.vue'
import RequestsPage from '../RequestsPage/RequestsPage.vue'
import ThreadPage from '../ThreadPage/ThreadPage.vue'
import DatasetsPage from '../DatasetsPage/DatasetsPage.vue'
import { getCookie } from '@/CookieService/index'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomePage
    },
    {
      path: '/messages',
      name: 'messages',
      children: [
        {
          path: 'new-request',
          name: 'new request',
          component: NewRequestPage
        },
        {
          path: 'requests',
          name: 'my requests',
          component: RequestsPage
        },
        {
          path: 'request/:id',
          name: 'request',
          component: ThreadPage
        }
      ]
    },
    {
      path: '/data-publishing',
      name: 'data publishing',
      children: [
        {
          path: 'datasets',
          name: 'datasets',
          component: DatasetsPage
        }
      ]
    },
    {
      path: '/login',
      name: 'login',
      component: LoginPage,
      meta: { sidebar: false }
    }
  ]
})

router.beforeEach((to, from, next) => {
  const token = getCookie('access_token')
  if (!token && to.name !== 'home') {
    return (window.location.href = 'https://github.com')
  }
  next()
})

export default router
