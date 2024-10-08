<script setup lang="ts">
import Drawer from 'primevue/drawer'
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { type User } from './types/user'
import type { PropType } from 'vue'

const currentRoute = useRoute()
const shouldShowSidebar = computed(() => currentRoute.meta.sidebar !== false)

defineProps({
  user: { required: true, type: Object as PropType<User> }
})

defineEmits(['logout'])

const navigationList = [
  {
    header: 'account',
    children: [
      {
        name: 'organizations',
        icon: 'pi pi-users mr-2',
        route: 'account/organizations'
      }
    ]
  },
  {
    header: 'data publishing',
    children: [
      {
        name: 'getting started',
        icon: 'pi pi-book mr-2',
        route: 'data-publishing/getting-started'
      },
      {
        name: 'lagoon',
        icon: 'pi pi-cloud mr-2',
        route: 'data-publishing/lagoon'
      },
      {
        name: 'datasets',
        icon: 'pi pi-database mr-2',
        route: 'data-publishing/datasets'
      }
    ]
  }
  // {
  //   header: 'messages',
  //   children: [
  //     {
  //       name: 'my requests',
  //       icon: 'pi pi-list mr-2',
  //       route: 'messages/requests'
  //     }
  //   ]
  // }
]
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
    pt:root:style="box-shadow: none;"
    pt:mask:style="width: auto;"
  >
    <template #container="{ closeCallback }">
      <div class="flex flex-col h-full gap-10">
        <div class="flex items-center pt-10">
          <span class="flex items-center gap-2 w-full justify-center">
            <img src="../../assets/images/tiki-logo-2.svg" alt="" class="w-1/12" />
            <span class="font-semibold text-2xl text-primary">TIKI</span>
          </span>
        </div>

        <div class="overflow-y-auto flex flex-col gap-6">
          <ul class="list-none px-8 m-0" v-for="section in navigationList">
            <li class="">
              <span class="font-bold uppercase">{{ section.header }}</span>
              <ul class="list-none p-0 m-0 overflow-hidden" v-for="route in section.children">
                <li>
                  <router-link :to="`/${route.route}`">
                    <a
                      class="flex items-center cursor-pointer p-2 rounded hover:bg-tiki-green/10"
                      :class="
                        currentRoute.name == route.name ? 'bg-tiki-green/10 text-tiki-green' : ''
                      "
                    >
                      <i :class="route.icon"></i>
                      <span class="font-medium capitalize">{{ route.name }}</span>
                    </a>
                  </router-link>
                </li>
              </ul>
            </li>
          </ul>
        </div>

        <div class="mt-auto">
          <hr class="border-t border border-black/10" />
          <div class="flex items-center justify-center p-6 gap-4">
            <span class="font-bold text-sm">{{ user.signInDetails.loginId }}</span>
            <button
              aria-label="logout"
              class="flex items-center justify-center hover:bg-black/10 p-1 rounded-md"
              @click="$emit('logout')"
            >
              <i class="pi pi-sign-out"></i>
            </button>
          </div>
        </div>
      </div>
    </template>
  </Drawer>
</template>

<style>
.p-drawer-mask {
  width: auto !important;
}

.p-drawer-left .p-drawer {
  width: 15rem;
}

td,
th {
  padding: 1rem 1rem;
}
</style>
