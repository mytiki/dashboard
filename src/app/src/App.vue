<script setup lang="ts">
import { RouterView } from 'vue-router'
import MainLayout from './MainLayout/MainLayout.vue'
import { onMounted } from 'vue'
import { setCookie, getCookie } from '@/CookieService/index'

onMounted(() => {
  const queryString = window.location.search
  const urlParams = new URLSearchParams(queryString)

  const accessTokenCookie = getCookie('access_token')

  if (accessTokenCookie) return

  const idToken = urlParams.get('id_token')
  const accessToken = urlParams.get('access_token')

  if (!idToken || !accessToken) return (window.location.href = 'https://github.com')

  setCookie('id_token', idToken!, 1)
  setCookie('access_token', accessToken!, 1)
})
</script>

<template>
  <MainLayout />
  <RouterView />
</template>
