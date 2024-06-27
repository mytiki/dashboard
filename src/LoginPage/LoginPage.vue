<script setup lang="ts">
import InputText from 'primevue/inputtext'
import FloatLabel from 'primevue/floatlabel'
import Password from 'primevue/password'
import Button from 'primevue/button'
import Image from 'primevue/image'

import { ref } from 'vue'

const email = ref<string>()
const password = ref<string>()
</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col items-center justify-center sm:w-1/3 gap-6 w-full">
      <img src="../assets/images/tiki-logo.svg" alt="" class="sm:hidden" width="150" />
      <div class="px-6 text-center">
        <h1 class="text-3xl">Welcome to Tiki Dashboard</h1>
        <h2 class="text-tiki-gray">Manage your data in a easy way</h2>
      </div>
      <div class="flex flex-col gap-6 w-1/2">
        <FloatLabel>
          <InputText id="email" v-model="email" class="w-full" />
          <label for="email">Email</label>
        </FloatLabel>
        <FloatLabel>
          <InputText
            v-model="password"
            id="password"
            type="password"
            :feedback="false"
            class="w-full"
          />
          <label for="password">Password</label>
        </FloatLabel>
        <Button label="Login" severity="success" />
      </div>
    </div>
    <div class="flex flex-col items-center justify-center w-2/3 bg-yellow-light hidden sm:flex">
      <img src="../assets/images/tiki-logo.svg" alt="" />
    </div>
  </div>
</template>

<style scoped></style>
