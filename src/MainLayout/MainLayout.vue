<script setup lang="ts">
import Drawer from 'primevue/drawer'
import Menu from 'primevue/menu'
import Avatar from 'primevue/avatar'
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { PrimeIcons } from '@primevue/core/api'

const route = useRoute()
const shouldShowSidebar = computed(() => route.meta.sidebar !== false)
</script>

<template>
  <Drawer
    :visible="true"
    header="Drawer"
    v-if="shouldShowSidebar"
    :modal="false"
    :showCloseIcon="false"
    :baseZIndex="0"
    :autoZIndex="false"
    pt:root:style="box-shadow: none; background-color: #FFF0BC;"
  >
    <template #container="{ closeCallback }">
      <div class="flex flex-col h-full">
        <div class="flex items-center px-6 pt-4 shrink-0 w-full">
          <span class="flex items-center gap-2 w-full justify-center">
            <img src="../assets/images/tiki-logo-2.svg" alt="" />
            <span class="font-semibold text-2xl text-primary">TIKI</span>
          </span>
        </div>

        <div class="overflow-y-auto">
          <ul class="list-none p-4 m-0">
            <li>
              <span class="font-bold">ACCOUNT</span>
              <ul class="list-none p-0 m-0 overflow-hidden">
                <li>
                  <router-link to="/organization">
                    <a class="flex items-center cursor-pointer p-4 rounded hover:bg-white">
                      <i class="pi pi-user mr-2"></i>
                      <span class="font-medium">Organization</span>
                    </a>
                  </router-link>
                </li>
                <li>
                  <router-link to="/team">
                    <a class="flex items-center cursor-pointer p-4 rounded hover:bg-white">
                      <i class="pi pi-users mr-2"></i>
                      <span class="font-medium">Team</span>
                    </a>
                  </router-link>
                </li>
                <li>
                  <router-link to="/billing">
                    <a class="flex items-center cursor-pointer p-4 rounded hover:bg-white">
                      <i class="pi pi-receipt mr-2"></i>
                      <span class="font-medium">Billing</span>
                    </a>
                  </router-link>
                </li>
              </ul>
            </li>
          </ul>
          <ul class="list-none p-4 m-0">
            <li>
              <span class="font-bold">DATA PUBLISHING</span>
            </li>
          </ul>
          <ul class="list-none p-4 m-0">
            <li>
              <span class="font-bold">MESSAGES</span>
            </li>
          </ul>
        </div>

        <div class="mt-auto">
          <hr class="mb-4 mx-4 border-t border-2 border-white" />
          <a class="m-4 flex items-center cursor-pointer p-4 gap-2 rounded">
            <Avatar
              image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png"
              shape="circle"
            />
            <span class="font-bold">Miro Benício</span>
          </a>
        </div>
      </div>
    </template>
  </Drawer>
</template>

<style scoped></style>
