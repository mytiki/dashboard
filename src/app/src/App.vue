<script setup lang="ts">
import { RouterView } from 'vue-router'
import MainLayout from './MainLayout/MainLayout.vue'
import { Authenticator, useAuthenticator } from '@aws-amplify/ui-vue'

import { cognitoUserPoolsTokenProvider } from 'aws-amplify/auth/cognito'
import { defaultStorage } from 'aws-amplify/utils'

cognitoUserPoolsTokenProvider.setKeyValueStorage(defaultStorage)

const auth = useAuthenticator()
</script>

<template>
  <div
    class="flex flex-col gap-4 items-center justify-center h-screen"
    v-if="auth.route !== 'authenticated'"
  >
    <img src="./assets/images/tiki-logo.svg" alt="" class="w-24" />
    <authenticator></authenticator>
  </div>

  <template v-if="auth.route === 'authenticated'">
    <MainLayout :user="auth.user" @logout="auth.signOut()" />
    <RouterView />
  </template>
</template>

<style>
[data-amplify-authenticator] {
  --amplify-components-button-primary-background-color: #00b272;
  --amplify-components-fieldcontrol-focus-box-shadow: #00b272;
  --amplify-components-tabs-item-active-border-color: #00b272;
  --amplify-components-tabs-item-color: var(--amplify-components-field-label-color);
  --amplify-components-tabs-item-hover-color: #00b272;
  --amplify-components-tabs-item-active-color: #00b272;
  --amplify-components-button-link-color: var(--amplify-components-field-label-color);
  --amplify-components-button-link-hover-color: #ffffff;
  --amplify-components-button-link-hover-background-color: #00b272;
  --amplify-components-button-primary-hover-background-color: #007048;
  --amplify-components-passwordfield-button-hover-background-color: #00b27210;
}
</style>
